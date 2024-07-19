// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::{IntoDiagnostic, Result};

pub struct ZipSaveDialog;

impl super::ZipWriteTarget for ZipSaveDialog {
    async fn write(&self, file_name: String, data: &[u8]) -> Result<()> {
        let saved = rfd::AsyncFileDialog::new()
            .set_title("Choose where to save the template")
            .add_filter("Zip file", &["zip"])
            .set_file_name(file_name)
            .save_file()
            .await;

        if let Some(file) = saved {
            file.write(data).await.into_diagnostic()?;
        }

        Ok(())
    }
}
