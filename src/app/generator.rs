// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::tap::Tap;
use crate::templates::*;
use crate::{MappingSet, ProjectType};
use bytes::Bytes;
use futures::future::join_all;
use futures::{join, FutureExt};
use miette::{IntoDiagnostic, Result};
use version_resolver::version_metadata::{JavaVersion, MinecraftVersionList};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use version_resolver::maven::{resolve_latest_version, resolve_matching_version, MavenLibrary};

pub async fn generate(app: &super::GeneratorApp, version_list: &MinecraftVersionList, filer_provider: &impl crate::filer::FilerProvider) -> Result<()> {
    let mut context = engine::Context::new();
    // Mod properties
    context.put("PACKAGE_NAME", &app.package_name);
    context.put("PACKAGE_DIR", &app.package_name.replace(".", "/"));
    let mod_id: String = app.get_effective_mod_id();
    context.put("MOD_ID", mod_id);
    let escaped_name = escape_json_and_toml(&app.mod_name);
    context.put("MOD_NAME", escaped_name);

    // Game version-specific
    let game_version = version_list.versions.iter()
        .find(|x| x.version == app.game_version)
        .unwrap();
    let java_version = JavaVersion::try_from(game_version.java_version).unwrap();
    context.put("MINECRAFT_VERSION", &game_version.version);
    context.put(
        "GRADLE_JAVA_VERSION",
        java_version.gradle_java_version(),
    );
    context.put(
        "JAVA_MAJOR_VERSION",
        game_version.java_version.to_string(),
    );
    context.put(
        "MIXIN_COMPAT_LEVEL",
        java_version.mixin_compat_level()
    );
    context.put("ARCHITECTURY_GROUP", &game_version.architectury.maven_group);
    context.put("ARCHITECTURY_PACKAGE", &game_version.architectury.package);
    context.put("FABRIC_API_MOD_ID", &game_version.fabric.fabric_api_mod_id);
    context.maybe_put(
        "FORGE_LOADER_MAJOR",
        game_version.forge.as_ref().map(|x| x.major_version)
    );
    context.maybe_put(
        "NEOFORGE_LOADER_MAJOR",
        game_version.neoforge.as_ref().map(|x| &x.loader_major_version),
    );
    context.maybe_put(
        "NEOFORGE_MAJOR",
        game_version.neoforge.as_ref().map(|x| &x.neoforge_major_version)
    );
    context.maybe_put(
        "FORGE_PACK_FORMAT",
        game_version.forge.as_ref().map(|x| &x.pack_version)
    );

    if let Some((data_pack_format_key, data_pack_format)) = game_version.forge.as_ref().and_then(|x| x.server_pack_version.as_ref()) {
        context.put("FORGE_DATA_PACK_FORMAT_KEY", data_pack_format_key);
        context.put("FORGE_DATA_PACK_FORMAT", data_pack_format);
    }

    // Constants
    context.put("LOOM_VERSION", crate::versions::LOOM_VERSION);
    context.put("PLUGIN_VERSION", crate::versions::PLUGIN_VERSION);

    // Setup version resolving
    let client = Arc::new(reqwest::ClientBuilder::new().build().into_diagnostic()?);
    let versions = crate::app::versions::get_version_index(client.clone(), &game_version).await?;
    let mut files: Vec<Pin<Box<dyn Future<Output = Result<Vec<FileData>>>>>> =
        vec![Box::pin(shared::shared_files(client.clone()))];
    let mut variables: Vec<Pin<Box<dyn Future<Output = Result<(String, String)>>>>> = Vec::new();

    // Mappings
    match app.mapping_set {
        MappingSet::Mojang => context.define("mojang_mappings"),
        MappingSet::Yarn => {
            context.define("yarn");
            variables.push(Box::pin(add_key(
                "YARN_MAPPINGS",
                resolve_matching_version(&client, MavenLibrary::yarn(), |version| {
                    version.starts_with(&format!("{}+", game_version.version))
                }),
            )));
        }
    }

    // Project-type specific
    match app.project_type {
        ProjectType::Multiplatform => {
            let mut platforms: Vec<&'static str> = vec![];
            files.push(Box::pin(multiplatform::all_files(client.clone())));
            variables.push(Box::pin(add_key(
                "FABRIC_LOADER_VERSION",
                resolve_latest_version(&client, MavenLibrary::fabric_loader()),
            )));

            if app.subprojects.fabric {
                context.define("fabric");
                files.push(Box::pin(fabric::all_files(client.clone())));
                variables.push(Box::pin(add_key(
                    "FABRIC_API_VERSION",
                    resolve_matching_version(&client, MavenLibrary::fabric_api(), |version| {
                        version.ends_with(&format!("+{}", game_version.fabric.fabric_api_branch.as_ref().unwrap_or(&game_version.version)))
                    }),
                )));
                platforms.push("fabric");
            }

            if app.subprojects.fabric_likes {
                context.define("fabric_like");
                files.push(Box::pin(fabric_like::all_files(client.clone())));
            }

            if app.subprojects.forge {
                context.define("forge");
                files.push(Box::pin(forge::all_files(client.clone())));
                if let Some(version) = versions.forge {
                    variables.push(Box::pin(add_key(
                        "FORGE_VERSION",
                        std::future::ready(Ok(version)),
                    )));
                }
                platforms.push("forge");
            }

            if app.subprojects.neoforge {
                context.define("neoforge");
                files.push(Box::pin(neoforge::main_files(client.clone())));
                if let Some(version) = versions.neoforge {
                    variables.push(Box::pin(add_key(
                        "NEOFORGE_VERSION",
                        std::future::ready(Ok(version)),
                    )));
                }
                if game_version.version == "1.20.4" {
                    context.put("NEOFORGE_METADATA_FILE_NAME", "mods.toml");
                    files.push(Box::pin(neoforge::mods_toml_files(client.clone())));
                } else {
                    context.put("NEOFORGE_METADATA_FILE_NAME", "neoforge.mods.toml");
                    files.push(Box::pin(neoforge::neoforge_mods_toml_files(client.clone())));
                }
                context.maybe_put("NEOFORGE_YARN_PATCH_VERSION", versions.neoforge_yarn_patch);
                platforms.push("neoforge");
            }

            if app.subprojects.quilt {
                context.define("quilt");
                files.push(Box::pin(quilt::all_files(client.clone())));
                variables.push(Box::pin(add_key(
                    "QUILT_LOADER_VERSION",
                    resolve_latest_version(&client, MavenLibrary::quilt_loader())
                )));
                variables.push(Box::pin(add_key(
                    "QUILTED_FABRIC_API_VERSION",
                    resolve_matching_version(&client, MavenLibrary::quilted_fabric_api(), |version| {
                        version.ends_with(&format!("-{}", game_version.version))
                    }),
                )));
                platforms.push("quilt");
            }

            let platforms = platforms.join(",");
            context.put("ARCHITECTURY_PLATFORMS", platforms);

            if app.dependencies.architectury_api {
                context.define("architectury_api");
                variables.push(Box::pin(add_key(
                    "ARCHITECTURY_API_VERSION",
                    std::future::ready(Ok(versions.architectury_api)),
                )));
            }
        }
        ProjectType::NeoForge => {
            files.push(Box::pin(neoforge_only::main_files(client.clone())));
            if let Some(version) = versions.neoforge {
                variables.push(Box::pin(add_key(
                    "NEOFORGE_VERSION",
                    std::future::ready(Ok(version)),
                )));
            }
            if game_version.version == "1.20.4" {
                context.put("NEOFORGE_METADATA_FILE_NAME", "mods.toml");
                files.push(Box::pin(neoforge_only::mods_toml_files(client.clone())));
            } else {
                context.put("NEOFORGE_METADATA_FILE_NAME", "neoforge.mods.toml");
                files.push(Box::pin(neoforge_only::neoforge_mods_toml_files(client.clone())));
            }
            context.maybe_put("NEOFORGE_YARN_PATCH_VERSION", versions.neoforge_yarn_patch);
        }
        ProjectType::Forge => {
            files.push(Box::pin(forge_only::all_files(client.clone())));
            if let Some(version) = versions.forge {
                variables.push(Box::pin(add_key(
                    "FORGE_VERSION",
                    std::future::ready(Ok(version)),
                )));
            }
        }
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

    filer_provider.use_filer(|filer| {
        let file_name = compose_file_name(app);
        filer.set_file_name(file_name);

        for file_data in files {
            let path = engine::apply_variables(&context, file_data.path.as_str(), false);
            let content: Bytes = match &file_data.content {
                FileContent::Binary(bytes) => bytes.clone(),
                FileContent::Text(text) => {
                    let applied: String =
                        engine::apply_template(&context, engine::read_template(text).unwrap())
                            .iter()
                            .map(|line| line.to_owned() + "\n")
                            .collect();
                    Bytes::from(applied)
                }
            };

            filer.save(path.as_str(), &content, &file_data.permissions).tap(|result| {
                if let Err(err) = result {
                    eprintln!("Could not save {}: {:?}", path, err);
                }
            })?;
        }

        Ok(())
    })
    .await
}

pub fn compose_file_name(app: &super::GeneratorApp) -> String {
    let mut file_name = app.get_effective_mod_id();
    file_name += "-";
    file_name += &app.game_version;

    match app.project_type {
        ProjectType::Multiplatform => {
            if app.subprojects.fabric && app.subprojects.quilt && app.subprojects.fabric_likes {
                file_name += "-fabric-like";
            } else {
                if app.subprojects.fabric {
                    file_name += "-fabric";
                }

                if app.subprojects.quilt {
                    file_name += "-quilt";
                }
            }

            if app.subprojects.neoforge {
                file_name += "-neoforge";
            }

            if app.subprojects.forge {
                file_name += "-forge";
            }
        },
        ProjectType::NeoForge => file_name += "-neoforge-only",
        ProjectType::Forge => file_name += "-forge-only",
    }

    file_name += "-template";
    file_name
}

fn add_key<F>(key: &'static str, future: F) -> impl Future<Output = Result<(String, String)>>
where
    F: Future<Output = Result<String>>,
{
    future.map(|result| result.map(|version| (key.to_owned(), version)))
}

/// Escapes a raw string so it can be embedded in a JSON or TOML quoted string value.
fn escape_json_and_toml(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for c in input.chars() {
        if c.is_ascii_control() {
            let c = c as u16;
            output.push_str(&format!("\\u{:04X}", c));
        } else {
            if c == '\\' || c == '"' {
                output.push('\\');
            }

            output.push(c);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn nothing_needs_escaping() {
        let input = "Hello, world‽ 🧶";
        let escaped = super::escape_json_and_toml(input);
        assert_eq!(escaped, input);
    }

    #[test]
    fn escape_quotes() {
        let input = "My \"Great\" Mod";
        let escaped = super::escape_json_and_toml(input);
        assert_eq!(escaped, "My \\\"Great\\\" Mod");
    }

    #[test]
    fn escape_backslashes() {
        let input = "My Mod \\ with a Weird Name";
        let escaped = super::escape_json_and_toml(input);
        assert_eq!(escaped, "My Mod \\\\ with a Weird Name");
    }

    #[test]
    fn escape_controls() {
        let input = "Hello\tWorld";
        let escaped = super::escape_json_and_toml(input);
        assert_eq!(escaped, "Hello\\u0009World");
    }
}
