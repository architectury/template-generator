// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod engine;
pub mod fabric;
pub mod forge;
pub mod multiplatform;

pub struct FileData {
    pub path: String,
    pub content: String,
}

pub fn compose_file_path(dir: &str, file_name: &str) -> String {
    let mut path = String::from(dir);
    if !dir.is_empty() {
        path += "/";
    }
    path += file_name;
    path
}

#[cfg(target_arch = "wasm32")]
pub async fn download_relative_file(client: std::sync::Arc<reqwest::Client>, url: &str) -> miette::Result<String> {
    use miette::{miette, IntoDiagnostic};
    use crate::web::ResultExt;

    let document = web_sys::window().ok_or_else(|| miette!("Could not find window"))?
        .document().ok_or_else(|| miette!("Could not find document"))?;
    let document_url = document.url().to_miette()?;
    let base_url = if let Some(slash_index) = document_url.rfind("/") {
        &document_url[0..=slash_index]
    } else {
        &document_url
    };
    let base = reqwest::Url::parse(base_url).into_diagnostic()?;
    let parsed_url = reqwest::Url::options().base_url(Some(&base)).parse(url).into_diagnostic()?;
    let response = client.get(parsed_url).send().await.into_diagnostic()?;

    if !response.status().is_success() {
        return Err(miette!(
            "Could not download {}: got status code {}",
            url,
            response.status()
        ));
    }

    response.text().await.into_diagnostic()
}

macro_rules! file_data {
    ($const_name:ident $fn_name:ident, $dir:expr, $file_name:expr) => {
        #[cfg(not(target_arch = "wasm32"))]
        const $const_name: &'static str = include_str!($file_name);

        #[cfg(not(target_arch = "wasm32"))]
        async fn $fn_name(
            _client: std::sync::Arc<reqwest::Client>,
        ) -> miette::Result<crate::templates::FileData> {
            let path = crate::templates::compose_file_path($dir, $file_name);
            Ok(crate::templates::FileData {
                path,
                content: $const_name.to_owned(),
            })
        }

        #[cfg(target_arch = "wasm32")]
        async fn $fn_name(
            client: std::sync::Arc<reqwest::Client>,
        ) -> miette::Result<crate::templates::FileData> {
            let path = crate::templates::compose_file_path($dir, $file_name);
            let dir = if $dir.is_empty() { "multiplatform" } else { $dir };
            let url = format!("templates/{}/{}", dir, $file_name);
            let content = crate::templates::download_relative_file(client, &url).await?;
            Ok(crate::templates::FileData { path, content })
        }
    };
}

macro_rules! file_list {
    ($vis:vis $fn_name:ident, $($file_fn:ident)+) => {
        $vis async fn $fn_name(client: std::sync::Arc<reqwest::Client>) -> miette::Result<Vec<crate::templates::FileData>> {
            let mut output: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = miette::Result<crate::templates::FileData>>>>> = Vec::new();
            $(
            output.push(Box::pin($file_fn(client.clone())));
            )+
            let results = futures::future::join_all(output).await;
            results.into_iter().collect()
        }
    }
}

pub(crate) use file_data;
pub(crate) use file_list;
