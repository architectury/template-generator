pub mod app;
pub mod focus;
pub mod screen;
pub mod widget;

use crate::versions::ALL_MINECRAFT_VERSIONS;
use app::*;
use crossterm::event::Event;
use focus::*;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use screen::*;
use std::cell::RefCell;
use std::io::Stderr;
use std::rc::Rc;
use tui_textarea::{Input, Key, TextArea};
use widget::*;

const HIGHLIGHTED_BLOCK_STYLE: Style = Style::new().fg(Color::LightBlue);

struct MainScreen<'a> {
    focus: Focus,
    mod_name_area: TextArea<'a>,
    mod_id_area: TextArea<'a>,
    package_name_area: TextArea<'a>,
    minecraft_version_dropdown: Dropdown,
}

impl<'a> MainScreen<'a> {
    fn new() -> Self {
        let minecraft_version_dropdown = Dropdown::new(
            ALL_MINECRAFT_VERSIONS
                .iter()
                .map(|version| (version.version, Style::default()))
                .collect(),
        );

        MainScreen {
            focus: Focus::new(4),
            mod_name_area: TextArea::default(),
            mod_id_area: TextArea::default(),
            package_name_area: TextArea::default(),
            minecraft_version_dropdown,
        }
    }

    fn focus_targets(&mut self) -> Vec<Option<&mut dyn Widget>> {
        vec![
            Some(&mut self.mod_name_area),
            Some(&mut self.mod_id_area),
            Some(&mut self.package_name_area),
            Some(&mut self.minecraft_version_dropdown),
        ]
    }
}

impl<'a> Screen for MainScreen<'a> {
    fn view(&self, f: &mut Frame<CrosstermBackend<Stderr>>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.size());
        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(layout[0]);

        let mod_name_block =
            Block::new()
                .title("Mod name")
                .borders(Borders::ALL)
                .style(
                    self.focus
                        .choose_at(0, HIGHLIGHTED_BLOCK_STYLE, Style::default()),
                );
        let mod_name_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(mod_name_block.inner(left_layout[0]));
        f.render_widget(mod_name_block, left_layout[0]);
        f.render_widget(
            Paragraph::new("The human-readable name of your mod."),
            mod_name_layout[0],
        );
        f.render_widget(self.mod_name_area.widget(), mod_name_layout[1]);

        let mod_id_block = Block::new()
            .title("Mod ID (optional)")
            .borders(Borders::ALL)
            .style(
                self.focus
                    .choose_at(1, HIGHLIGHTED_BLOCK_STYLE, Style::default()),
            );
        let mod_id_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(mod_id_block.inner(left_layout[1]));
        f.render_widget(mod_id_block, left_layout[1]);
        f.render_widget(
            Paragraph::new("A unique ID for your mod."),
            mod_id_layout[0],
        );
        f.render_widget(self.mod_id_area.widget(), mod_id_layout[1]);

        let package_name_block = Block::new()
            .title("Package name")
            .borders(Borders::ALL)
            .style(
                self.focus
                    .choose_at(2, HIGHLIGHTED_BLOCK_STYLE, Style::default()),
            );
        let package_name_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(1), Constraint::Length(1)])
            .split(package_name_block.inner(left_layout[2]));
        f.render_widget(package_name_block, left_layout[2]);
        f.render_widget(
            Paragraph::new("A unique package name for your mod."),
            package_name_layout[0],
        );
        f.render_widget(self.package_name_area.widget(), package_name_layout[1]);

        let minecraft_version_block = Block::new()
            .title("Minecraft version")
            .borders(Borders::ALL)
            .style(
                self.focus
                    .choose_at(3, HIGHLIGHTED_BLOCK_STYLE, Style::default()),
            );
        f.render_widget(
            self.minecraft_version_dropdown.widget(),
            minecraft_version_block.inner(left_layout[3]),
        );
        f.render_widget(minecraft_version_block, left_layout[3]);

        let mappings_block = Block::new().title("Mappings").borders(Borders::ALL);
        let mappings_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(mappings_block.inner(left_layout[4]));
        f.render_widget(mappings_block, left_layout[4]);
        f.render_widget(
            Paragraph::new("The set of names used for Minecraft code."),
            mappings_layout[0],
        );
        f.render_widget(
            Paragraph::new("The official obfuscation maps published by Mojang."),
            mappings_layout[2],
        );
    }

    fn input(&mut self, event: Event) -> Option<Message> {
        // Note: the text area crate's Input used here to prevent rapid repeated keystrokes
        let input: Input = event.clone().into();
        match input {
            Input { key: Key::Tab, .. } => {
                self.focus.cycle();
                None
            }
            Input { key: Key::Esc, .. } => Some(Message::CloseScreen),
            _ => {
                let selected_focus_target = self.focus.selected();

                if let Some(widget) = &mut self.focus_targets()[selected_focus_target] {
                    widget.input(event)
                } else {
                    None
                }
            }
        }
    }
}

pub fn create_app(terminal: Terminal<CrosstermBackend<Stderr>>) -> App {
    let mut app = App::new(terminal);
    app.push_screen(Rc::new(RefCell::new(MainScreen::new())));
    app
}
