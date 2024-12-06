// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use eyre::Result;
use std::collections::HashSet;
use std::io::{Cursor, Seek, Write};
use zip::write::SimpleFileOptions;

// Platform impls
#[cfg(not(target_family = "wasm"))]
pub mod native;
#[cfg(target_family = "wasm")]
pub mod web;

pub trait Filer {
    fn set_file_name(&mut self, file_name: String);
    fn save(&mut self, path: &str, content: &[u8], permissions: &FilePermissions) -> Result<()>;
}

#[allow(async_fn_in_trait)]
pub trait FilerProvider {
    async fn use_filer<F>(&self, block: F) -> Result<()>
    where
        F: FnOnce(&mut dyn Filer) -> Result<()>;
}

pub enum FilePermissions {
    None,
    Execute,
}

impl FilePermissions {
    pub fn unix(&self) -> u32 {
        match self {
            Self::None => 0o644,
            Self::Execute => 0o755,
        }
    }
}

pub struct ZipFiler<'a, W>
where
    W: Write + Seek,
{
    writer: &'a mut zip::ZipWriter<W>,
    directories: HashSet<String>,
    pub file_name: Option<String>,
}

impl<'a, W> ZipFiler<'a, W>
where
    W: Write + Seek,
{
    pub fn new(writer: &'a mut zip::ZipWriter<W>) -> Self {
        Self {
            writer,
            directories: HashSet::new(),
            file_name: None,
        }
    }
}

impl<'a, W> Filer for ZipFiler<'a, W>
where
    W: Write + Seek,
{
    fn save(&mut self, path: &str, content: &[u8], permissions: &FilePermissions) -> Result<()> {
        let parts: Vec<_> = path.split("/").collect();

        for i in 0..(parts.len() - 1) {
            let directory = format!("{}/", parts[0..=i].join("/"));

            if self.directories.insert(directory.clone()) {
                self.writer.add_directory(directory, SimpleFileOptions::default())?;
            }
        }

        self.writer.start_file(path, SimpleFileOptions::default().unix_permissions(permissions.unix()))?;
        self.writer.write_all(content)?;
        Ok(())
    }

    fn set_file_name(&mut self, file_name: String) {
        self.file_name = Some(file_name)
    }
}

pub struct ZipFilerProvider<T: ZipWriteTarget>(pub T);

impl<T: ZipWriteTarget> FilerProvider for ZipFilerProvider<T> {
    async fn use_filer<F>(&self, block: F) -> Result<()>
    where
        F: FnOnce(&mut dyn Filer) -> Result<()> {
        let mut cursor = Cursor::new(Vec::new());
        let mut file_name: String = "template.zip".to_owned();

        // Create and use the zip writer and filer.
        // This is its own scope in order to drop the borrow to the cursor.
        {
            let mut writer = zip::ZipWriter::new(&mut cursor);
            {
                let mut filer = ZipFiler::new(&mut writer);
                block(&mut filer)?;

                if let Some(custom_name) = filer.file_name {
                    file_name = custom_name + ".zip";
                }
            }
            writer.finish()?;
        }

        self.0.write(file_name, cursor.get_ref()).await
    }
}

#[allow(async_fn_in_trait)]
pub trait ZipWriteTarget {
    async fn write(&self, file_name: String, data: &[u8]) -> Result<()>;
}
