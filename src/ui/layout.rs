// TUI layout: URL bar, content area, status bar.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

fn content_lines(app: &App) -> Text<'_> {
    let mut lines: Vec<Line> = Vec::new();
    let link_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::UNDERLINED);
    let selected_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::REVERSED | Modifier::BOLD);

    if let Some(page) = &app.page {
        for t in &page.text {
            lines.push(Line::from(t.as_str()));
        }
        if !page.links.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from("--- Links ---"));
            for (i, (anchor, _)) in page.links.iter().enumerate() {
                let label = if anchor.is_empty() {
                    "[link]"
                } else {
                    anchor.as_str()
                };
                let line_text = format!("[{}] {}", i + 1, label);
                let line = if i == app.selected_link {
                    Line::from(line_text).style(selected_style)
                } else {
                    Line::from(line_text).style(link_style)
                };
                lines.push(line);
            }
        }
    }

    Text::from(lines)
}

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
    let content = Paragraph::new(content_lines(app))
        .block(content_block)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));
    frame.render_widget(content, chunks[1]);

    let link_count = match app.page.as_ref() {
        Some(p) => p.links.len(),
        None => 0,
    };
    let link_index = if link_count == 0 {
        0
    } else {
        app.selected_link + 1
    };
    let status_text = format!(
        "{} | scroll: {} | link: {}/{} | [↑↓ scroll] [j/k select link] [Enter open] [b back] [f forward] [q quit]",
        app.current_url,
        app.scroll,
        link_index,
        link_count
    );
    let status_block = Block::default()
        .title(" Status ")
        .borders(Borders::ALL)
        .border_style(Style::default());
    let status = Paragraph::new(status_text).block(status_block);
    frame.render_widget(status, chunks[2]);
}
