// Input handling: map key events to actions.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use crate::app::App;

/// Poll for a key event and update app state. Sets app.should_quit on 'q'.
/// Arrow Up/Down adjust scroll (scroll never goes negative).
pub fn handle_input(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            match key.code {
                KeyCode::Char('q') => app.should_quit = true,
                KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
                KeyCode::Down => app.scroll += 1,
                _ => {}
            }
        }
    }
    Ok(())
}
