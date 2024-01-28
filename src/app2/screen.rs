// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cell::RefCell;
use std::io::Stderr;
use std::rc::Rc;

use crossterm::event::Event;
use ratatui::prelude::CrosstermBackend;
use ratatui::Frame;

pub enum Message {
    OpenScreen(Rc<RefCell<dyn Screen>>),
    CloseScreen,
}

pub trait Screen {
    fn view(&self, f: &mut Frame<CrosstermBackend<Stderr>>);
    fn input(&mut self, event: Event) -> Option<Message>;
}
