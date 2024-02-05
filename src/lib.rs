// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod app;
#[cfg(not(target_family = "wasm"))]
pub mod app2;
#[cfg(not(target_family = "wasm"))]
pub mod async_support;
pub mod mod_ids;
pub mod tap;
pub mod templates;
#[cfg(target_family = "wasm")]
pub mod web;

pub use app::*;
