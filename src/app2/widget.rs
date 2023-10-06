use crossterm::event::Event;
use tui_textarea::TextArea;

pub trait Widget {
    fn input(&mut self, event: Event) -> Option<super::screen::Message>;
}

impl<'a> Widget for TextArea<'a> {
    fn input(&mut self, event: Event) -> Option<super::screen::Message> {
        TextArea::input(self, event);
        None
    }
}
