// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub struct Focus {
    count: usize,
    selected: usize,
}

impl Focus {
    pub fn new(count: usize) -> Self {
        Focus { count, selected: 0 }
    }

    pub fn cycle(&mut self) {
        self.selected = (self.selected + 1) % self.count;
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected == index
    }

    pub fn choose_at<T>(&self, index: usize, a: T, b: T) -> T {
        if self.is_selected(index) {
            a
        } else {
            b
        }
    }
}
