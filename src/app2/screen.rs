use std::io::Stderr;
use ratatui::Frame;
use ratatui::prelude::CrosstermBackend;
use tui_textarea::Input;

pub enum Message {
    OpenScreen(Box<dyn Screen>),
    CloseScreen
}

pub trait Screen {
    fn view(&self, f: &mut Frame<CrosstermBackend<Stderr>>);
    fn input(&mut self, input: Input) -> Option<Message>;
}
