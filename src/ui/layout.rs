// TUI layout: URL bar, content area, status bar.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Render the main TUI layout with URL bar, content area, and status bar.
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    let url_bar = Block::default()
        .title(" URL ")
        .borders(Borders::ALL)
        .border_style(Style::default());
    let url_content = Paragraph::new(app.current_url.as_str()).block(url_bar);
    frame.render_widget(url_content, chunks[0]);

    let content_block = Block::default()
        .title(" Content ")
        .borders(Borders::ALL)
        .border_style(Style::default());
    let content = Paragraph::new("CONTENT AREA").block(content_block);
    frame.render_widget(content, chunks[1]);

    let status_block = Block::default()
        .title(" Status ")
        .borders(Borders::ALL)
        .border_style(Style::default());
    let status = Paragraph::new("").block(status_block);
    frame.render_widget(status, chunks[2]);
}
