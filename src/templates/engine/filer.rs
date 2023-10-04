use miette::Result;

pub trait Filer {
    fn save(&self, path: &str, content: &str) -> Result<()>;
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::use_filer;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use miette::{IntoDiagnostic, Result};
    use rfd::FileDialog;
    use std::{fs, path};

    pub fn use_filer<F>(block: F) -> Result<()>
    where
        F: FnOnce(&dyn super::Filer) -> Result<()>,
    {
        let saved = FileDialog::new()
            .set_title("Choose where to save the template")
            .pick_folder();

        if let Some(directory) = saved {
            let filer = DirectoryFiler {
                path: directory.as_ref(),
            };
            block(&filer)?;
        }

        // TODO: Return error when cancelled
        Ok(())
    }

    struct DirectoryFiler<'a> {
        path: &'a path::Path,
    }

    impl<'a> super::Filer for DirectoryFiler<'a> {
        fn save(&self, path: &str, content: &str) -> Result<()> {
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

#[cfg(target_arch = "wasm32")]
pub use web::use_filer;

#[cfg(target_arch = "wasm32")]
mod web {
    use miette::Result;

    pub fn use_filer<F>(block: F) -> Result<()>
    where
        F: FnOnce(&dyn super::Filer) -> Result<()>,
    {
        todo!()
    }
}
