# clibro

A terminal-based web browser written in Rust.

## Minimal CLI Browser

- **Fetch** a webpage from a URL (blocking HTTP)
- **Parse** HTML and extract headings (h1–h3), paragraphs, and links
- **Render** text in the terminal; links shown as a numbered list
- **Navigate** by entering a link number to follow, or `q` to quit

No TUI (no ratatui). Plain terminal I/O.

### Run

```bash
cargo run
```

Then enter a URL (e.g. `https://example.com`), then a link number to follow or `q` to quit.

### Build & check

```bash
cargo build
cargo clippy
```

### Layout

```
src/
├── main.rs
└── browser/
    ├── mod.rs
    ├── fetcher.rs
    ├── parser.rs
    └── renderer.rs
```

### Dependencies

- `reqwest` (blocking)
- `scraper`
- `anyhow`
- `url`
