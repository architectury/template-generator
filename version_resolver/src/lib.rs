// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod index;
pub mod maven;
pub mod version_metadata;
#[cfg(target_family = "wasm")]
pub(crate) mod web;
pub(crate) mod xml;
