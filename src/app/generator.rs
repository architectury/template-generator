// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::tap::Tap;
use crate::templates::*;
use crate::{MappingSet, ProjectType};
use futures::future::join_all;
use futures::{FutureExt, join};
use miette::{IntoDiagnostic, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use version_resolver::maven::{resolve_latest_version, resolve_matching_version};

const FABRIC_MAVEN: &'static str = "https://maven.fabricmc.net";
const ARCHITECTURY_MAVEN: &'static str = "https://maven.architectury.dev";

pub async fn generate(app: &super::GeneratorApp) -> Result<()> {
    let mut context = engine::Context::new();
    // Mod properties
    context.put("PACKAGE_NAME", &app.package_name);
    context.put("PACKAGE_DIR", &app.package_name.replace(".", "/"));
    let mut mod_id: String = app.mod_id.clone();
    if mod_id.is_empty() {
        mod_id = crate::mod_ids::to_mod_id(&app.mod_name);
    }
    context.put("MOD_ID", mod_id);
    // TODO: Escape
    context.put("MOD_NAME_JSON", &app.mod_name);
    context.put("MOD_NAME_TOML", &app.mod_name);

    // Game version-specific
    let game_version = app.game_version;
    context.put("MINECRAFT_VERSION", game_version.version());
    context.put(
        "GRADLE_JAVA_VERSION",
        game_version.java_version().gradle_java_version(),
    );
    context.put(
        "JAVA_MAJOR_VERSION",
        game_version.java_version().java_major_version().to_string(),
    );
    context.put("FORGE_LOADER_MAJOR", game_version.forge_major_version());
    context.maybe_put("NEOFORGE_LOADER_MAJOR", game_version.neoforge_loader_major());
    context.maybe_put("NEOFORGE_MAJOR", game_version.neoforge_major());

    // Constants
    context.put("LOOM_VERSION", crate::versions::LOOM_VERSION);
    context.put("PLUGIN_VERSION", crate::versions::PLUGIN_VERSION);

    // Setup version resolving
    let client = Arc::new(reqwest::ClientBuilder::new().build().into_diagnostic()?);
    let mut files: Vec<Pin<Box<dyn Future<Output = Result<Vec<FileData>>>>>> = vec![Box::pin(shared::shared_files(client.clone()))];
    let mut variables: Vec<Pin<Box<dyn Future<Output = Result<(String, String)>>>>> = Vec::new();

    // Mappings
    match app.mapping_set {
        MappingSet::Mojang => context.define("mojang_mappings"),
        MappingSet::Yarn => {
            context.define("yarn");
            variables.push(Box::pin(add_key("YARN_MAPPINGS", resolve_matching_version(
                &client,
                FABRIC_MAVEN,
                "net.fabricmc",
                "yarn",
                |version| version.starts_with(&format!("{}+", game_version.version())),
            ))));
        }
    }

    // Project-type specific
    match app.project_type {
        ProjectType::Multiplatform => {
            files.push(Box::pin(multiplatform::all_files(client.clone())));
            variables.push(Box::pin(add_key("FABRIC_LOADER_VERSION", resolve_latest_version(
                &client,
                FABRIC_MAVEN,
                "net.fabricmc",
                "fabric-loader",
            ))));

            if app.subprojects.fabric {
                context.define("fabric");
                files.push(Box::pin(fabric::all_files(client.clone())));
                variables.push(Box::pin(add_key("FABRIC_API_VERSION", resolve_matching_version(
                    &client,
                    FABRIC_MAVEN,
                    "net.fabricmc.fabric-api",
                    "fabric-api",
                    |version| version.ends_with(&format!("+{}", game_version.fabric_api_branch())),
                ))));
            }

            if app.subprojects.forge {
                context.define("forge");
                files.push(Box::pin(forge::all_files(client.clone())));
            }

            if app.subprojects.neoforge {
                context.define("neoforge");
                files.push(Box::pin(neoforge::all_files(client.clone())));
            }

            if app.subprojects.quilt {
                context.define("quilt");
            }

            if app.dependencies.architectury_api {
                context.define("architectury_api");
                variables.push(Box::pin(add_key("ARCHITECTURY_API_VERSION", resolve_matching_version(
                    &client,
                    ARCHITECTURY_MAVEN,
                    game_version.architectury_package(),
                    "architectury",
                    |version| {
                        version
                            .starts_with(&format!("{}.", game_version.architectury_api_version()))
                    },
                ))));
            }
        }
        ProjectType::NeoForge => {}
        ProjectType::Forge => {}
    }

    // Resolve versions
    let (files, variables) = join!(join_all(files), join_all(variables));
    let files: Vec<FileData> = files
        .into_iter()
        .collect::<Result<Vec<Vec<FileData>>>>()?
        .into_iter()
        .flatten()
        .collect();
    for result in variables {
        let (key, value) = result?;
        context.put(key, value);
    }

    engine::filer::use_filer(|filer| {
        for file_data in files {
            let path = engine::apply_variables(&context, file_data.path.as_str(), false);
            let content: String = engine::apply_template(
                &context,
                engine::read_template(&file_data.content).unwrap(),
            )
            .iter()
            .map(|line| line.to_owned() + "\n")
            .collect();
            filer.save(path.as_str(), content.as_str()).tap(|result| {
                if let Err(err) = result {
                    eprintln!("Could not save {}: {:?}", path, err);
                }
            })?;
        }

        Ok(())
    })
    .await
}

fn add_key<F>(key: &'static str, future: F) -> impl Future<Output = Result<(String, String)>>
where
    F: Future<Output = Result<String>>,
{
    future.map(|result| result.map(|version| (key.to_owned(), version)))
}
