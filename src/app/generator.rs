use crate::requests::*;
use crate::tap::Tap;
use crate::templates::*;
use crate::{MappingSet, ProjectType};
use futures::future::join_all;
use futures::join;
use miette::{miette, IntoDiagnostic, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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

    // Constants
    context.put("LOOM_VERSION", crate::versions::LOOM_VERSION);
    context.put("PLUGIN_VERSION", crate::versions::PLUGIN_VERSION);

    // Setup version resolving
    let mut files: Vec<Pin<Box<dyn Future<Output = Result<Vec<FileData>>>>>> = Vec::new();
    let client = Arc::new(reqwest::ClientBuilder::new().build().into_diagnostic()?);
    let mut variables: Vec<Pin<Box<dyn Future<Output = Result<(String, String)>>>>> = Vec::new();

    // Mappings
    match app.mapping_set {
        MappingSet::Mojang => context.define("mojang_mappings"),
        MappingSet::Yarn => {
            context.define("yarn");
            variables.push(Box::pin(resolve_matching_version(
                &client,
                "YARN_MAPPINGS",
                FABRIC_MAVEN,
                "net.fabricmc",
                "yarn",
                |version| version.starts_with(&format!("{}+", game_version.version())),
            )));
        }
    }

    // Project-type specific
    match app.project_type {
        ProjectType::Multiplatform => {
            files.push(Box::pin(multiplatform::all_files(client.clone())));
            variables.push(Box::pin(resolve_latest_version(
                &client,
                "FABRIC_LOADER_VERSION",
                FABRIC_MAVEN,
                "net.fabricmc",
                "fabric-loader",
            )));

            if app.subprojects.fabric {
                context.define("fabric");
                files.push(Box::pin(fabric::all_files(client.clone())));
                variables.push(Box::pin(resolve_matching_version(
                    &client,
                    "FABRIC_API_VERSION",
                    FABRIC_MAVEN,
                    "net.fabricmc.fabric-api",
                    "fabric-api",
                    |version| version.ends_with(&format!("+{}", game_version.fabric_api_branch())),
                )));
            }

            if app.subprojects.forge {
                context.define("forge");
                files.push(Box::pin(forge::all_files(client.clone())));
            }

            if app.subprojects.neoforge {
                context.define("neoforge");
            }

            if app.subprojects.quilt {
                context.define("quilt");
            }

            if app.dependencies.architectury_api {
                context.define("architectury_api");
                variables.push(Box::pin(resolve_matching_version(
                    &client,
                    "ARCHITECTURY_API_VERSION",
                    ARCHITECTURY_MAVEN,
                    game_version.architectury_package(),
                    "architectury",
                    |version| {
                        version.starts_with(&format!("{}.", game_version.architectury_api_version()))
                    },
                )));
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
    }).await
}

async fn resolve_matching_version<F>(
    client: &reqwest::Client,
    variable: &str,
    repository: &str,
    group: &str,
    name: &str,
    filter: F,
) -> Result<(String, String)>
where
    F: Fn(&str) -> bool,
{
    let metadata = download_maven_metadata(client, repository, group, name).await?;
    let version = get_latest_version_matching(&metadata, filter).ok_or_else(|| {
        miette!(
            "Could not find latest version for {}:{} in {}",
            group,
            name,
            repository
        )
    })?;
    Ok((variable.to_owned(), version))
}

async fn resolve_latest_version(
    client: &reqwest::Client,
    variable: &str,
    repository: &str,
    group: &str,
    name: &str,
) -> Result<(String, String)> {
    let metadata = download_maven_metadata(client, repository, group, name).await?;
    let version = get_latest_version(&metadata).ok_or_else(|| {
        miette!(
            "Could not find latest version for {}:{} in {}",
            group,
            name,
            repository
        )
    })?;
    Ok((variable.to_owned(), version))
}
