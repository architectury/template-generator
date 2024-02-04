// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::xml::{read_node, XmlNode};
use miette::{miette, IntoDiagnostic, Result};
use reqwest::Client;

async fn download_maven_metadata(
    client: &Client,
    repository: &str,
    group: &str,
    artifact: &str,
) -> Result<impl XmlNode> {
    let url = format!(
        "{}/{}/{}/maven-metadata.xml",
        repository,
        group.replace(".", "/"),
        artifact
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
    repository: &str,
    group: &str,
    name: &str,
    filter: F,
) -> Result<String>
where
    F: Fn(&str) -> bool,
{
    let metadata = download_maven_metadata(client, repository, group, name).await?;
    get_latest_version_matching(&metadata, filter).ok_or_else(|| {
        miette!(
            "Could not find latest version for {}:{} in {}",
            group,
            name,
            repository
        )
    })
}

pub async fn resolve_latest_version(
    client: &reqwest::Client,
    repository: &str,
    group: &str,
    name: &str,
) -> Result<String> {
    let metadata = download_maven_metadata(client, repository, group, name).await?;
    get_latest_version(&metadata).ok_or_else(|| {
        miette!(
            "Could not find latest version for {}:{} in {}",
            group,
            name,
            repository
        )
    })
}
