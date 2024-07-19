// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::{miette, Context, IntoDiagnostic, Result};
use std::{fs, path};

pub struct DirectoryFilerProvider<'a>(pub &'a path::Path);

impl<'a> super::FilerProvider for DirectoryFilerProvider<'a> {
    async fn use_filer<F>(&self, block: F) -> Result<()>
    where
        F: FnOnce(&mut dyn super::Filer) -> Result<()>,
    {
        block(&mut DirectoryFiler { path: self.0 })
    }
}

struct DirectoryFiler<'a> {
    path: &'a path::Path,
}

#[cfg(target_family = "unix")]
fn update_permissions(path: &path::Path, permissions: &super::FilePermissions) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let file_permissions = fs::metadata(path).into_diagnostic()?.permissions();
    let new_mode = file_permissions.mode() | permissions.unix();
    let new_permissions = Permissions::from_mode(new_mode);
    fs::set_permissions(path, new_permissions).into_diagnostic()
}

#[cfg(not(target_family = "unix"))]
fn update_permissions(_path: &path::Path, _permissions: &super::FilePermissions) -> Result<()> {
    // Not supported on Windows
    Ok(())
}

impl<'a> super::Filer for DirectoryFiler<'a> {
    fn save(
        &mut self,
        path: &str,
        content: &[u8],
        permissions: &super::FilePermissions,
    ) -> Result<()> {
        let mut full_path = path::PathBuf::from(self.path);
        full_path.push(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).into_diagnostic()?;
        }

        fs::write(&full_path, content).into_diagnostic()?;
        update_permissions(&full_path, permissions)?;
        Ok(())
    }

    fn set_file_name(&mut self, _file_name: String) {
        // Don't do anything as the directory filer prompts for the output name.
    }
}

pub enum FsZipWriteTarget {
    ZipFile(path::PathBuf),
    InDirectory(path::PathBuf),
}

impl super::ZipWriteTarget for FsZipWriteTarget {
    async fn write(&self, file_name: String, data: &[u8]) -> Result<()> {
        let path = match self {
            Self::ZipFile(path) => path,
            Self::InDirectory(path) => &path.join(file_name),
        };

        let exists = tokio::fs::try_exists(path)
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("Could not check if file {} exists", path.to_string_lossy()))?;
        if exists {
            return Err(miette!("Output file {} already exists!", path.to_string_lossy()));
        }

        tokio::fs::write(path, data).await.into_diagnostic()
    }
}
