// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    version_list: std::path::PathBuf,
    #[arg(short, long, value_name = "FILE")]
    output: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    use miette::IntoDiagnostic;
    use templateer::versions::{index::VersionIndex, version_metadata::MinecraftVersionList};

    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let list: MinecraftVersionList = serde_json::from_str(std::fs::read_to_string(cli.version_list).into_diagnostic()?.as_str()).into_diagnostic()?;
    let index = VersionIndex::resolve(&client, &list).await?;
    let json = serde_json::to_string_pretty(&index).into_diagnostic()?;
    let path = cli
        .output
        .unwrap_or(std::path::PathBuf::from("version_index.json"));

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).into_diagnostic()?;
        }
    }

    std::fs::write(path, json).into_diagnostic()?;
    Ok(())
}
