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

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        ui::handle_input(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    terminal.show_cursor()?;
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
