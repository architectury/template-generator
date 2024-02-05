// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::{IntoDiagnostic, Result};
use rfd::AsyncFileDialog;
use std::collections::HashSet;
use std::io::{Cursor, Seek, Write};
use zip::write::FileOptions;

pub trait Filer {
    fn save(&mut self, path: &str, content: &[u8]) -> Result<()>;
}

pub struct ZipFiler<'a, W>
where
    W: Write + Seek,
{
    writer: &'a mut zip::ZipWriter<W>,
    directories: HashSet<String>,
}

impl<'a, W> ZipFiler<'a, W>
where
    W: Write + Seek,
{
    pub fn new(writer: &'a mut zip::ZipWriter<W>) -> Self {
        Self {
            writer,
            directories: HashSet::new(),
        }
    }
}

impl<'a, W> Filer for ZipFiler<'a, W>
where
    W: Write + Seek,
{
    fn save(&mut self, path: &str, content: &[u8]) -> Result<()> {
        let parts: Vec<_> = path.split("/").collect();

        for i in 0..(parts.len() - 1) {
            let directory = format!("{}/", parts[0..=i].join("/"));

            if self.directories.insert(directory.clone()) {
                self.writer
                    .add_directory(directory, FileOptions::default())
                    .into_diagnostic()?;
            }
        }

        self.writer
            .start_file(path, FileOptions::default())
            .into_diagnostic()?;
        self.writer.write_all(content).into_diagnostic()?;
        Ok(())
    }
}

#[cfg(not(target_family = "wasm"))]
pub use native::use_filer;

#[cfg(not(target_family = "wasm"))]
mod native {
    use miette::{IntoDiagnostic, Result};
    use rfd::FileDialog;
    use std::{fs, path};

    pub async fn use_filer<F>(block: F) -> Result<()>
    where
        F: FnOnce(&mut dyn super::Filer) -> Result<()>,
    {
        let saved = FileDialog::new()
            .set_title("Choose where to save the template")
            .pick_folder();

        if let Some(directory) = saved {
            let mut filer = DirectoryFiler {
                path: directory.as_ref(),
            };
            block(&mut filer)?;
        }

        // TODO: Return error when cancelled
        Ok(())
    }

    struct DirectoryFiler<'a> {
        path: &'a path::Path,
    }

    impl<'a> super::Filer for DirectoryFiler<'a> {
        fn save(&mut self, path: &str, content: &[u8]) -> Result<()> {
            let mut full_path = path::PathBuf::from(self.path);
            full_path.push(path);

            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).into_diagnostic()?;
            }

            fs::write(full_path, content).into_diagnostic()?;
            Ok(())
        }
    }
}

pub async fn use_zip_filer<F>(block: F) -> Result<()>
where
    F: FnOnce(&mut dyn Filer) -> Result<()>,
{
    let mut cursor = Cursor::new(Vec::new());

    // Create and use the zip writer and filer.
    // This is its own scope in order to drop the borrow to the cursor.
    {
        let mut writer = zip::ZipWriter::new(&mut cursor);
        {
            let mut filer = ZipFiler::new(&mut writer);
            block(&mut filer)?;
        }
        writer.finish().into_diagnostic()?;
    }

    let saved = AsyncFileDialog::new()
        .set_title("Choose where to save the template")
        .add_filter("Zip file", &["zip"])
        .set_file_name("template.zip")
        .save_file()
        .await;

    if let Some(file) = saved {
        file.write(cursor.get_ref()).await.into_diagnostic()?;
    }

    Ok(())
}

#[cfg(target_family = "wasm")]
pub use web::use_filer;

#[cfg(target_family = "wasm")]
mod web {
    use miette::Result;

    pub async fn use_filer<F>(block: F) -> Result<()>
    where
        F: FnOnce(&mut dyn super::Filer) -> Result<()>,
    {
        super::use_zip_filer(block).await
    }
}
