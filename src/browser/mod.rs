mod fetcher;
mod history;
mod parser;
mod renderer;

pub use fetcher::fetch;
pub use history::History;
pub use parser::parse;
pub use parser::Page;
pub use renderer::render;
