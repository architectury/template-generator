// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use miette::{IntoDiagnostic, Result};
use ratatui::prelude::*;
use templateer::app2::create_app;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<()> {
    // Set up Crossterm
    crossterm::terminal::enable_raw_mode().into_diagnostic()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)
        .into_diagnostic()?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr())).into_diagnostic()?;
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
    crossterm::terminal::disable_raw_mode().into_diagnostic()?;

    Ok(())
}
