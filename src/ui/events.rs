// Input handling: map key events to actions.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

/// Outcome of handling input: quit requested or continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputResult {
    Quit,
    Continue,
}

/// Poll for a key event and handle it. Returns Quit on 'q', else Continue.
pub fn handle_input() -> std::io::Result<InputResult> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    return Ok(InputResult::Quit);
                }
            }
        }
    }
    Ok(InputResult::Continue)
}
