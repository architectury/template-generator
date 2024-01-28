pub mod engine;
pub mod fabric;
pub mod forge;
pub mod multiplatform;

pub struct FileData {
    pub path: String,
    pub content: String,
}

macro_rules! file_data {
    ($const_name:ident $fn_name:ident, $dir:expr, $file_name:expr) => {
        #[cfg(not(target_arch = "wasm32"))]
        const $const_name: &'static str = include_str!($file_name);

        #[cfg(not(target_arch = "wasm32"))]
        async fn $fn_name(
            _client: std::sync::Arc<reqwest::Client>,
        ) -> miette::Result<crate::templates::FileData> {
            let mut path = String::from($dir);
            if !$dir.is_empty() {
                path += "/";
            }
            path += $file_name;
            Ok(crate::templates::FileData {
                path,
                content: $const_name.to_owned(),
            })
        }

        #[cfg(target_arch = "wasm32")]
        async fn $fn_name(
            client: std::sync::Arc<reqwest::Client>,
        ) -> miette::Result<crate::templates::FileData> {
            use miette::{IntoDiagnostic, miette};

            let mut path = String::from("templates/");
            if !$dir.is_empty() {
                path += $dir;
                path += "/";
            }
            path += $file_name;

            let response = client.get(&path).send().await.into_diagnostic()?;
            if !response.status().is_success() {
                return Err(miette!(
                    "Could not download {}: got status code {}",
                    path,
                    response.status()
                ));
            }

            let content = response.text().await.into_diagnostic()?;
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
