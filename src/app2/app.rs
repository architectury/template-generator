use miette::{IntoDiagnostic, Result};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use tui_textarea::Input;
use super::screen::{Message, Screen};

pub struct App {
    terminal: Terminal<CrosstermBackend<std::io::Stderr>>,
    screen_stack: Vec<Box<dyn Screen>>,
    should_exit: bool,
}

impl App {
    pub fn new(terminal: Terminal<CrosstermBackend<std::io::Stderr>>) -> Self {
        App {
            terminal,
            screen_stack: Vec::new(),
            should_exit: false,
        }
    }

    pub fn push_screen(&mut self, screen: Box<dyn Screen>) {
        self.screen_stack.push(screen);
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn tick(&mut self) -> Result<()> {
        if let Some(screen) = self.screen_stack.last_mut() {
            self.terminal.draw(|f| screen.view(f))
                .into_diagnostic()?;

            let input: Input = crossterm::event::read().into_diagnostic()?.into();
            if let Some(message) = screen.input(input) {
                match message {
                    Message::OpenScreen(next) => self.screen_stack.push(next),
                    Message::CloseScreen => {
                        self.screen_stack.pop();
                        if self.screen_stack.is_empty() {
                            self.should_exit = true;
                        }
                    },
                }
            }
        }

        Ok(())
    }
}
