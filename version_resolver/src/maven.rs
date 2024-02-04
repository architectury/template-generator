// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::xml::{read_node, XmlNode};
use miette::{miette, IntoDiagnostic, Result};
use reqwest::Client;

const FABRIC_MAVEN: &'static str = "https://maven.fabricmc.net";
const ARCHITECTURY_MAVEN: &'static str = "https://maven.architectury.dev";

pub struct MavenLibrary {
    repository: MavenRepository,
    group: String,
    name: String,
}

impl MavenLibrary {
    pub fn new<S>(repository: MavenRepository, group: S, name: S) -> Self
    where
        S: AsRef<str>,
    {
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
        Self::new(MavenRepository::Fabric, "net.fabricmc.fabric-api", "fabric-api")
    }

    // Architectury libraries
    pub fn architectury_api() -> Self {
        Self::new(MavenRepository::Architectury, "dev.architectury", "architectury-api")
    }
}

impl std::fmt::Display for MavenLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} in {}", self.group, self.name, self.repository.url())
    }
}

pub enum MavenRepository {
    Fabric,
    Architectury,
}

impl MavenRepository {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Fabric => FABRIC_MAVEN,
            Self::Architectury => ARCHITECTURY_MAVEN,
        }
    }

    pub fn allows_cross_origin(&self) -> bool {
        match self {
            Self::Fabric => true,
            Self::Architectury => false,
        }
    }
}

async fn download_maven_metadata(
    client: &Client,
    library: &MavenLibrary,
) -> Result<impl XmlNode> {
    let url = format!(
        "{}/{}/{}/maven-metadata.xml",
        library.repository().url(),
        library.group().replace(".", "/"),
        library.name()
    );
    let response = client.get(&url).send().await.into_diagnostic()?;

    if !response.status().is_success() {
        return Err(miette!(
            "Could not download {}: got status code {}",
            url,
            response.status()
        ));
    }

    let text = response.text().await.into_diagnostic()?;
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
    get_latest_version_matching(&metadata, filter).ok_or_else(|| {
        miette!("Could not find latest version for {}", library)
    })
}

pub async fn resolve_latest_version(
    client: &reqwest::Client,
    library: MavenLibrary,
) -> Result<String> {
    let metadata = download_maven_metadata(client, &library).await?;
    get_latest_version(&metadata).ok_or_else(|| {
        miette!("Could not find latest version for {}", library)
    })
}
