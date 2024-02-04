// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    output: Option<std::path::PathBuf>
}

#[tokio::main]
#[cfg(not(target_arch = "wasm32"))]
async fn main() -> miette::Result<()> {
    use version_resolver::index::VersionIndex;

    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let index = VersionIndex::resolve(&client).await?;
    let json = serde_json::to_string_pretty(&index).map_err(|err| miette::miette!("{}", err))?;
    let path = cli.output.unwrap_or(std::path::PathBuf::from("version_index.json"));

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|err| miette::miette!("{}", err))?;
        }
    }

    std::fs::write(path, json).map_err(|err| miette::miette!("{}", err))?;
    Ok(())
}
