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
