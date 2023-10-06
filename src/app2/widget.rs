use std::cell::RefCell;
use std::io::Stderr;
use std::rc::Rc;

use crossterm::event::Event;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Style;
use ratatui::widgets::{List, ListItem, ListState, Paragraph, Widget as RWidget};
use ratatui::Frame;
use tui_textarea::{Input, Key, TextArea};

use crate::app2::screen::Screen;

use super::screen::Message;

pub trait Widget {
    fn input(&mut self, event: Event) -> Option<Message>;
}

impl<'a> Widget for TextArea<'a> {
    fn input(&mut self, event: Event) -> Option<Message> {
        TextArea::input(self, event);
        None
    }
}

pub struct Dropdown {
    items: Vec<(&'static str, Style)>,
    state: Rc<RefCell<ListState>>,
}

impl Dropdown {
    pub fn new(items: Vec<(&'static str, Style)>) -> Self {
        Self {
            items,
            state: Rc::new(RefCell::new(ListState::default().with_selected(Some(0)))),
        }
    }

    pub fn widget(&self) -> impl RWidget {
        let state = self.state.borrow();
        if let Some(selected) = state.selected() {
            let item_index = selected.min(self.items.len() - 1);
            let (text, style) = &self.items[item_index];
            Paragraph::new(*text).style(style.clone())
        } else {
            Paragraph::new("<none selected>")
        }
    }
}

impl Widget for Dropdown {
    fn input(&mut self, event: Event) -> Option<Message> {
        let input: Input = event.into();
        match input {
            Input {
                key: Key::Enter, ..
            } => {
                let list_items: Vec<_> = self
                    .items
                    .iter()
                    .map(|(text, style)| ListItem::new(*text).style(style.clone()))
                    .collect();
                let screen = DropdownScreen {
                    list: List::new(list_items),
                    state: self.state.clone(),
                };
                Some(Message::OpenScreen(Rc::new(RefCell::new(screen))))
            }
            _ => None,
        }
    }
}

struct DropdownScreen<'a> {
    list: List<'a>,
    state: Rc<RefCell<ListState>>,
}

impl<'a> Screen for DropdownScreen<'a> {
    fn view(&self, f: &mut Frame<CrosstermBackend<Stderr>>) {
        let state = self.state.borrow();
        f.render_stateful_widget(self.list.clone(), f.size(), &mut state.clone());
    }

    fn input(&mut self, event: Event) -> Option<Message> {
        let mut state = self.state.borrow_mut();

        let input: Input = event.into();
        match input {
            Input { key: Key::Up, .. } => {
                if let Some(selected) = state.selected() {
                    state.select(Some(selected.saturating_sub(1)))
                }
                None
            }
            Input { key: Key::Down, .. } => {
                if let Some(selected) = state.selected() {
                    let new_index = selected.saturating_add(1).min(self.list.len() - 1);
                    state.select(Some(new_index))
                }
                None
            }
            Input {
                key: Key::Enter, ..
            } => Some(Message::CloseScreen),
            _ => None,
        }
    }
}
