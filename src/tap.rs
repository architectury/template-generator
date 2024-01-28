// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait Tap {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self) -> ();
}

impl<T> Tap for T {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self) -> (),
    {
        f(&self);
        self
    }
}
