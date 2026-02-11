# CliBro

A terminal-based web browser written in Rust. It fetches HTML pages, parses headings and links, and lets you browse in a TUI with keyboard navigation.

## How It Works

1. **Start** — The app opens in a full-screen TUI and loads a default page (e.g. `https://example.com`).
2. **Fetch** — Blocking HTTP (reqwest) fetches the raw HTML for a URL.
3. **Parse** — The scraper extracts `h1`, `h2`, `h3`, `p` as text and `a[href]` as links; relative URLs are resolved to the current page URL.
4. **Render** — ratatui draws three areas: URL bar, scrollable content (text + numbered links), and a status bar with hints.
5. **Input** — Key events (crossterm) update app state: scroll, selected link, or trigger navigation (follow link, back, forward, quit).
6. **Navigate** — Following a link pushes the current URL onto the back stack and loads the new page; back/forward pop from the stacks and load without pushing.

No JavaScript, no images, no bookmarks or tabs—just text and links.

## Run

```bash
cargo run
```

## Build & check

```bash
cargo build
cargo clippy
```

Rust toolchain is set in `rust-toolchain.toml` (stable).

## Keyboard controls

| Key     | Action              |
|--------|---------------------|
| `↑`/`↓` | Scroll content      |
| `j`/`k` | Next / previous link |
| `Enter` | Follow selected link |
| `b`    | Back                |
| `f`    | Forward             |
| `q`    | Quit                |

## Layout (source)

```
src/
├── main.rs          # Entry, terminal setup, panic hook, event loop, navigation wiring
├── app.rs           # Centralized app state (URL, page, scroll, selection, history)
├── browser/
│   ├── mod.rs       # Re-exports
│   ├── fetcher.rs   # Blocking HTTP fetch
│   ├── parser.rs    # HTML → Page (text + links)
│   └── history.rs   # Back/forward stacks
└── ui/
    ├── mod.rs       # Re-exports draw, handle_input, InputResult
    ├── layout.rs    # ratatui layout and widgets (URL bar, content, status)
    └── events.rs    # Key polling → InputResult + scroll/link selection
```

## Dependencies

- **reqwest** — blocking HTTP, rustls (no native TLS)
- **scraper** — HTML parsing (selectors)
- **anyhow** — error handling
- **url** — URL parsing and resolution
- **ratatui** — TUI widgets and layout
- **crossterm** — terminal raw mode, alternate screen, key events

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture details, coding standards, and how to contribute.
