// Navigation history: back and forward stacks.

/// Back/forward navigation state.
#[derive(Debug, Default, Clone)]
pub struct History {
    pub back_stack: Vec<String>,
    pub forward_stack: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self::default()
    }

    /// Go back: move current URL to forward stack, pop from back stack. Returns None if empty.
    pub fn back(&mut self, current_url: String) -> Option<String> {
        let url = self.back_stack.pop()?;
        self.forward_stack.push(current_url);
        Some(url)
    }

    /// Go forward: move current URL to back stack, pop from forward stack. Returns None if empty.
    pub fn forward(&mut self, current_url: String) -> Option<String> {
        let url = self.forward_stack.pop()?;
        self.back_stack.push(current_url);
        Some(url)
    }
}
