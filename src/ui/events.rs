// Input handling: map key events to actions.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use crate::app::App;

/// Result of handling input: quit, follow a link, or continue.
#[derive(Debug)]
pub enum InputResult {
    Quit,
    Continue,
    FollowLink(String),
}

/// Poll for a key event and update app state. Returns FollowLink(url) when Enter on a valid link.
pub fn handle_input(app: &mut App) -> std::io::Result<InputResult> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(InputResult::Continue);
            }
            match key.code {
                KeyCode::Char('q') => return Ok(InputResult::Quit),
                KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
                KeyCode::Down => app.scroll += 1,
                KeyCode::Char('j') => {
                    let max_link = match app.page.as_ref() {
                        Some(p) => p.links.len().saturating_sub(1),
                        None => 0,
                    };
                    app.selected_link = (app.selected_link + 1).min(max_link);
                }
                KeyCode::Char('k') => {
                    app.selected_link = app.selected_link.saturating_sub(1);
                }
                KeyCode::Enter => {
                    if let Some(page) = &app.page {
                        if let Some((_, url)) = page.links.get(app.selected_link) {
                            return Ok(InputResult::FollowLink(url.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(InputResult::Continue)
}
