// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use eyre::{eyre, Result};

pub trait ResultExt<T> {
    fn to_eyre(self) -> Result<T>;
}

impl<T> ResultExt<T> for Result<T, wasm_bindgen::JsValue> {
    fn to_eyre(self) -> Result<T> {
        self.map_err(|err| eyre!("{:?}", err))
    }
}
