use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};
use tui_textarea::{Input, Key, TextArea};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    // Set up Crossterm
    // TODO: Is raw mode needed? https://ratatui.rs/concepts/backends/raw-mode.html
    // TODO: https://github.com/veeso/tui-realm-stdlib
    crossterm::terminal::enable_raw_mode()
        .into_diagnostic()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .into_diagnostic()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))
        .into_diagnostic()?;
    let mut text_area = TextArea::default();
    text_area.set_block(Block::default().title("My text"));

    loop {
        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(f.size());

            f.render_widget(Paragraph::new("Hello, world"), layout[0]);
            f.render_widget(text_area.widget(), layout[1]);
        }).into_diagnostic()?;

        match crossterm::event::read().into_diagnostic()?.into() {
            Input { key: Key::Tab, .. } => todo!(),
            input => text_area.input(input),
        };
    }

    // Clean up
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)
        .into_diagnostic()?;
    crossterm::terminal::disable_raw_mode()
        .into_diagnostic()?;

    Ok(())
}
