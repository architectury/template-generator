// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bytes::Bytes;
use crate::filer::FilePermissions;

pub mod engine;
pub mod fabric;
pub mod fabric_like;
pub mod forge;
pub mod forge_only;
pub mod multiplatform;
pub mod neoforge;
pub mod neoforge_only;
pub mod quilt;
pub mod shared;

pub struct FileData {
    pub path: String,
    pub content: FileContent,
    pub permissions: FilePermissions,
}

pub enum FileContent {
    Binary(Bytes),
    Text(String),
}

pub fn compose_file_path(dir: &str, file_name: &str, include_dir: bool) -> String {
    if include_dir && !dir.is_empty() {
        format!("{}/{}", dir, file_name)
    } else {
        String::from(file_name)
    }
}

#[cfg(target_family = "wasm")]
pub fn compose_relative_url(url: &str) -> crate::Result<reqwest::Url> {
    use crate::err;
    use crate::result::Error;

    let document = web_sys::window()
        .ok_or_else(|| err!("Could not find window"))?
        .document()
        .ok_or_else(|| err!("Could not find document"))?;
    let document_url = document.url().map_err(Error::from_js)?;
    let base_url = if let Some(slash_index) = document_url.rfind("/") {
        &document_url[0..=slash_index]
    } else {
        &document_url
    };
    let base = reqwest::Url::parse(base_url)?;
    let parsed_url = reqwest::Url::options()
        .base_url(Some(&base))
        .parse(url)?;
    Ok(parsed_url)
}

#[cfg(target_family = "wasm")]
pub async fn download_relative_text(
    client: std::sync::Arc<reqwest::Client>,
    url: &str,
) -> crate::Result<String> {
    use crate::err;

    let parsed_url = compose_relative_url(url)?;
    let response = client.get(parsed_url).send().await?;

    if !response.status().is_success() {
        return Err(err!(
            "Could not download {}: got status code {}",
            url,
            response.status()
        ));
    }

    Ok(response.text().await?)
}

#[cfg(target_family = "wasm")]
pub async fn download_relative_binary(
    client: std::sync::Arc<reqwest::Client>,
    url: &str,
) -> crate::Result<Bytes> {
    use crate::err;

    let parsed_url = compose_relative_url(url)?;
    let response = client.get(parsed_url).send().await?;

    if !response.status().is_success() {
        return Err(err!(
            "Could not download {}: got status code {}",
            url,
            response.status()
        ));
    }

    Ok(response.bytes().await?)
}

macro_rules! file_data {
    ($const_name:ident $fn_name:ident, $dir:expr, $include_dir_in_target:expr, $file_name:expr) => {
        crate::templates::file_data!($const_name $fn_name, $dir, $include_dir_in_target, $file_name, None);
    };

    ($const_name:ident $fn_name:ident, $dir:expr, $include_dir_in_target:expr, $file_name:expr, $permissions:ident) => {
        crate::templates::file_data_raw!($const_name, $fn_name, $dir, $include_dir_in_target, $file_name, $permissions, str, include_str, Text, download_relative_text);
    };
}

macro_rules! file_data_with_target {
    ($const_name:ident $fn_name:ident,
     $dir:expr, $include_dir:expr,
     $source_name:expr, $target_name:expr) => {
        crate::templates::file_data_with_target!{
            $const_name, $fn_name, $dir, $include_dir, $source_name, $target_name, None
        }
    };

    ($const_name:ident, $fn_name:ident,
     $dir:expr, $include_dir:expr,
     $source_name:expr, $target_name:expr,
     $permissions:ident) => {
        #[cfg(not(target_family = "wasm"))]
        const $const_name: &'static str = include_str!($source_name);

        #[cfg(not(target_family = "wasm"))]
        async fn $fn_name(
            _client: std::sync::Arc<reqwest::Client>,
        ) -> crate::Result<crate::templates::FileData> {
            let path = crate::templates::compose_file_path(
                $dir, $target_name, $include_dir
            );
            Ok(crate::templates::FileData {
                path,
                content: crate::templates::FileContent::Text($const_name.into()),
                permissions: crate::filer::FilePermissions::$permissions,
            })
        }

        #[cfg(target_family = "wasm")]
        async fn $fn_name(
            client: std::sync::Arc<reqwest::Client>
        ) -> crate::Result<crate::templates::FileData> {
            let url = format!("templates/{}/{}", $dir.replace("-", "_"), $source_name);
            let bytes = crate::templates::download_relative_text(client, &url).await?;
            let content = crate::templates::FileContent::Text(bytes);
            let permissions = crate::filer::FilePermissions::$permissions;
            let path = crate::templates::compose_file_path(
                $dir, $target_name, $include_dir
            );
            Ok(crate::templates::FileData { path, content, permissions })
        }
    };
}


macro_rules! binary_file_data {
    ($const_name:ident $fn_name:ident, $dir:expr, $include_dir_in_target:expr, $file_name:expr) => {
        crate::templates::file_data_raw!($const_name, $fn_name, $dir, $include_dir_in_target, $file_name, None, [u8], include_bytes, Binary, download_relative_binary);
    };
}

macro_rules! file_data_raw {
    ($const_name:ident, $fn_name:ident, $dir:expr, $include_dir_in_target:expr, $file_name:expr, $permissions:ident,
        $static_type:ty, $static_include:ident, $file_content_type:ident, $download_function:ident) => {
        #[cfg(not(target_family = "wasm"))]
        const $const_name: &'static $static_type = $static_include!($file_name);

        #[cfg(not(target_family = "wasm"))]
        async fn $fn_name(
            _client: std::sync::Arc<reqwest::Client>,
        ) -> crate::Result<crate::templates::FileData> {
            let path =
                crate::templates::compose_file_path($dir, $file_name, $include_dir_in_target);
            Ok(crate::templates::FileData {
                path,
                content: crate::templates::FileContent::$file_content_type($const_name.into()),
                permissions: crate::filer::FilePermissions::$permissions,
            })
        }

        #[cfg(target_family = "wasm")]
        async fn $fn_name(
            client: std::sync::Arc<reqwest::Client>,
        ) -> crate::Result<crate::templates::FileData> {
            let path =
                crate::templates::compose_file_path($dir, $file_name, $include_dir_in_target);
            let url = format!("templates/{}/{}", $dir.replace("-", "_"), $file_name);
            let bytes = crate::templates::$download_function(client, &url).await?;
            let content = crate::templates::FileContent::$file_content_type(bytes);
            let permissions = crate::filer::FilePermissions::$permissions;
            Ok(crate::templates::FileData { path, content, permissions })
        }
    };
}

macro_rules! file_list {
    ($vis:vis $fn_name:ident, $($file_fn:ident)+) => {
        $vis async fn $fn_name(client: std::sync::Arc<reqwest::Client>) -> crate::Result<Vec<crate::templates::FileData>> {
            let mut output: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = crate::Result<crate::templates::FileData>>>>> = Vec::new();
            $(
            output.push(Box::pin($file_fn(client.clone())));
            )+
            let results = futures::future::join_all(output).await;
            results.into_iter().collect()
        }
    }
}

pub(crate) use binary_file_data;
pub(crate) use file_data;
pub(crate) use file_data_with_target;
pub(crate) use file_data_raw;
pub(crate) use file_list;
