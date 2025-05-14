// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::versions::version_metadata::MinecraftVersion;
use crate::xml::{read_node, XmlNode};
use crate::{err, Result};
use reqwest::Client;

const FABRIC_MAVEN: &'static str = "https://maven.fabricmc.net";
const ARCHITECTURY_MAVEN: &'static str = "https://maven.architectury.dev";
const FORGE_MAVEN: &'static str = "https://maven.minecraftforge.net";
const NEOFORGE_MAVEN: &'static str = "https://maven.neoforged.net/releases";
const QUILT_MAVEN: &'static str = "https://maven.quiltmc.org/repository/release/";

pub struct MavenLibrary {
    repository: MavenRepository,
    group: String,
    name: String,
}

impl MavenLibrary {
    pub fn new(repository: MavenRepository, group: impl AsRef<str>, name: impl AsRef<str>) -> Self {
        Self {
            repository,
            group: group.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
        }
    }

    pub fn repository(&self) -> &MavenRepository {
        &self.repository
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Fabric libraries
    pub fn yarn() -> Self {
        Self::new(MavenRepository::Fabric, "net.fabricmc", "yarn")
    }

    pub fn fabric_loader() -> Self {
        Self::new(MavenRepository::Fabric, "net.fabricmc", "fabric-loader")
    }

    pub fn fabric_api() -> Self {
        Self::new(
            MavenRepository::Fabric,
            "net.fabricmc.fabric-api",
            "fabric-api",
        )
    }

    // Architectury libraries
    pub fn architectury_api(game_version: &MinecraftVersion) -> Self {
        Self::new(
            MavenRepository::Architectury,
            &game_version.architectury.maven_group,
            "architectury",
        )
    }

    pub fn neoforge_yarn_patch() -> Self {
        Self::new(MavenRepository::Architectury, "dev.architectury", "yarn-mappings-patch-neoforge")
    }

    // Forge libraries
    pub fn forge() -> Self {
        Self::new(MavenRepository::Forge, "net.minecraftforge", "forge")
    }

    // NeoForge libraries
    pub fn neoforge() -> Self {
        Self::new(MavenRepository::NeoForge, "net.neoforged", "neoforge")
    }

    // Quilt libraries
    pub fn quilt_loader() -> Self {
        Self::new(MavenRepository::Quilt, "org.quiltmc", "quilt-loader")
    }

    pub fn quilted_fabric_api() -> Self {
        Self::new(MavenRepository::Quilt, "org.quiltmc.quilted-fabric-api", "quilted-fabric-api")
    }
}

impl std::fmt::Display for MavenLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} in {}",
            self.group,
            self.name,
            self.repository.url()
        )
    }
}

pub enum MavenRepository {
    Fabric,
    Architectury,
    Forge,
    NeoForge,
    Quilt,
}

impl MavenRepository {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Fabric => FABRIC_MAVEN,
            Self::Architectury => ARCHITECTURY_MAVEN,
            Self::Forge => FORGE_MAVEN,
            Self::NeoForge => NEOFORGE_MAVEN,
            Self::Quilt => QUILT_MAVEN,
        }
    }
}

async fn download_maven_metadata(client: &Client, library: &MavenLibrary) -> Result<impl XmlNode> {
    let url = format!(
        "{}/{}/{}/maven-metadata.xml",
        library.repository().url(),
        library.group().replace(".", "/"),
        library.name()
    );
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(err!(
            "Could not download {}: got status code {}",
            url,
            response.status()
        ));
    }

    let text = response.text().await?;
    read_node(text.as_str())
}

fn get_latest_version<N: XmlNode>(node: &N) -> Option<String> {
    node.get_first_child("metadata")?
        .get_first_child("versioning")?
        .get_first_child("latest")?
        .text()
}

fn get_latest_version_matching<N, F>(node: &N, filter: F) -> Option<String>
where
    N: XmlNode,
    F: Fn(&str) -> bool,
{
    let mut matching: Vec<String> = node
        .get_first_child("metadata")?
        .get_first_child("versioning")?
        .get_first_child("versions")?
        .get_children("version")
        .filter_map(|child| child.text())
        .filter(|version| filter(version.as_str()))
        .collect();
    matching.sort_by(|a, b| flexver_rs::compare(a.as_str(), b.as_str()).reverse());
    matching.first().cloned()
}

pub async fn resolve_matching_version<F>(
    client: &reqwest::Client,
    library: MavenLibrary,
    filter: F,
) -> Result<String>
where
    F: Fn(&str) -> bool,
{
    let metadata = download_maven_metadata(client, &library).await?;
    get_latest_version_matching(&metadata, filter)
        .ok_or_else(|| err!("Could not find latest version for {}", library))
}

pub async fn resolve_latest_version(
    client: &reqwest::Client,
    library: MavenLibrary,
) -> Result<String> {
    let metadata = download_maven_metadata(client, &library).await?;
    get_latest_version(&metadata)
        .ok_or_else(|| err!("Could not find latest version for {}", library))
}
