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
}
