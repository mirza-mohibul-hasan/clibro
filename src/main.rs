// Phase 2: TUI browser with ratatui.

mod app;
mod browser;
mod ui;

use app::App;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io;

/// Navigate to url: push current to history back stack, clear forward stack, fetch and parse.
fn navigate_to(app: &mut App, url: &str) {
    if !app.current_url.is_empty() {
        app.history.back_stack.push(app.current_url.clone());
        app.history.forward_stack.clear();
    }
    let html = match browser::fetch(url) {
        Ok(body) => body,
        Err(_) => return,
    };
    let page = match browser::parse(&html, url) {
        Ok(p) => p,
        Err(_) => return,
    };
    app.current_url = url.to_string();
    app.page = Some(page);
    app.scroll = 0;
    app.selected_link = 0;
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    navigate_to(&mut app, "https://example.com");

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        match ui::handle_input(&mut app)? {
            ui::InputResult::Quit => break,
            ui::InputResult::FollowLink(url) => navigate_to(&mut app, &url),
            ui::InputResult::Continue => {}
        }
    }

    terminal.show_cursor()?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
