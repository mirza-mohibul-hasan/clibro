// Centralized application state.

use crate::browser::{History, Page};

/// Main application state. All navigation and UI state lives here.
pub struct App {
    pub current_url: String,
    pub page: Option<Page>,
    pub scroll: u16,
    pub selected_link: usize,
    pub history: History,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_url: String::new(),
            page: None,
            scroll: 0,
            selected_link: 0,
            history: History::new(),
            should_quit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}
