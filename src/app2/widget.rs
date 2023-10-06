use crossterm::event::Event;
use tui_textarea::TextArea;

pub trait Widget {
    fn input(&mut self, event: Event);
}

impl<'a> Widget for TextArea<'a> {
    fn input(&mut self, event: Event) {
        TextArea::input(self, event);
    }
}
