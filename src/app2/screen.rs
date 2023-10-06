use std::io::Stderr;

use crossterm::event::Event;
use ratatui::Frame;
use ratatui::prelude::CrosstermBackend;

pub enum Message {
    OpenScreen(Box<dyn Screen>),
    CloseScreen
}

pub trait Screen {
    fn view(&self, f: &mut Frame<CrosstermBackend<Stderr>>);
    fn input(&mut self, event: Event) -> Option<Message>;
}
