use templateer::app2::create_app;
use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    // Set up Crossterm
    crossterm::terminal::enable_raw_mode()
        .into_diagnostic()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .into_diagnostic()?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))
        .into_diagnostic()?;
    let mut app = create_app(terminal);

    loop {
        app.tick()?;
        if app.should_exit() {
            break;
        }
    }

    // Clean up
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)
        .into_diagnostic()?;
    crossterm::terminal::disable_raw_mode()
        .into_diagnostic()?;

    Ok(())
}
