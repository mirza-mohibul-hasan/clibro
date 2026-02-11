// Input handling: map key events to actions.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use crate::app::App;

/// Poll for a key event and update app state. Sets app.should_quit on 'q'.
pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                app.should_quit = true;
            }
        }
    }
    Ok(())
}
