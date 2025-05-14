// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error as StdError;
use std::fmt::Display;

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        crate::result::Error::from(format!($($arg)*))
    };
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(Box<dyn StdError>);

impl Error {
    #[cfg(target_family = "wasm")]
    pub fn from_js(value: wasm_bindgen::JsValue) -> Self {
        format!("{:?}", value).into()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Into<Box<dyn StdError>>> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

pub trait ResultContext {
    type ReturnedResult;

    fn wrap_err(self, context: impl Display) -> Self::ReturnedResult;

    fn wrap_err_with<C: Display>(self, context: impl FnOnce() -> C) -> Self::ReturnedResult;
}

impl<T, E: Display> ResultContext for Result<T, E> {
    type ReturnedResult = Result<T, Error>;

    fn wrap_err(self, context: impl Display) -> Self::ReturnedResult {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::from(format!("{}: {}", context, error))),
        }
    }
    
    fn wrap_err_with<C: Display>(self, context: impl FnOnce() -> C) -> Self::ReturnedResult {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(Error::from(format!("{}: {}", context(), error))),
        }
    }
}
