# Contributing to CliBro

This guide describes the architecture, how the project works end-to-end, and how to contribute.

---

## Architecture overview

CliBro is a **Phase 2** TUI browser: one process, one “tab,” no async. All state lives in a single `App`; the UI only reads from it and emits input results. Navigation and fetch/parse are driven from `main` in response to input.

```
┌─────────────────────────────────────────────────────────────────┐
│  main.rs                                                         │
│  • Terminal lifecycle (raw mode, alternate screen, panic hook)   │
│  • Event loop: draw(frame, app) → handle_input(app) → act on      │
│    InputResult (Quit / FollowLink / Back / Forward / Continue)    │
│  • Navigation: load_page(), navigate_to() — calls browser::fetch │
│    and browser::parse, updates App                               │
└─────────────────────────────────────────────────────────────────┘
           │                    │                    │
           ▼                    ▼                    ▼
┌──────────────┐    ┌──────────────────┐    ┌──────────────────┐
│  app.rs      │    │  browser/        │    │  ui/             │
│  App state   │◄───│  fetcher, parser │    │  layout, events  │
│  (single     │    │  history         │    │  (read-only app,  │
│   source of  │    │  (no UI deps)    │    │   output actions) │
│   truth)     │    └──────────────────┘    └──────────────────┘
└──────────────┘
```

- **App** is the only place that holds URL, page, scroll, selected link, history, and quit flag.
- **browser/** does fetch, parse, and history; it has no knowledge of the TUI or key bindings.
- **ui/** only renders from `App` and turns key events into `InputResult`; it does not fetch, parse, or mutate history.

---

## Module-by-module

### `src/main.rs`

- **Terminal setup**: enable raw mode, enter alternate screen, create `Terminal<CrosstermBackend<Stdout>>`.
- **Panic hook**: on panic, restore terminal (show cursor, leave alternate screen, disable raw mode), then run the default panic handler.
- **Event loop** (in `run_app`):
  1. `terminal.draw(|frame| ui::draw(frame, &app))`
  2. `ui::handle_input(&mut app)` → `InputResult`
  3. Act on result: **Quit** → break; **FollowLink(url)** → `navigate_to(&mut app, &url)`; **Back** / **Forward** → `app.history.back/forward(...)` then `load_page(...)`; **Continue** → nothing.
- **Navigation helpers**:
  - `load_page(app, url)`: `browser::fetch(url)` → `browser::parse(html, url)` → set `app.current_url`, `app.page`, reset `scroll` and `selected_link`. Does **not** touch history.
  - `navigate_to(app, url)`: if there is a current URL, push it onto `history.back_stack` and clear `history.forward_stack`; then call `load_page(app, url)`.

No business logic lives in `main` beyond wiring: “this input result means this navigation action.”

---

### `src/app.rs`

**App** is the single source of truth:

- `current_url: String` — address of the current page.
- `page: Option<Page>` — parsed text and links; `None` before first load or on error.
- `scroll: u16` — vertical scroll offset for the content area (0 = top).
- `selected_link: usize` — index into `page.links` for the highlighted link (0-based).
- `history: History` — back and forward stacks (URLs).
- `should_quit: bool` — set by input handler when user presses `q` (used implicitly via `InputResult::Quit` in the loop).

All UI and navigation decisions are derived from these fields; nothing is cached in the UI layer.

---

### `src/browser/`

Pure “browser engine” layer: no ratatui, no crossterm.

- **fetcher.rs**  
  - `fetch(url: &str) -> Result<String>`  
  - Blocking GET with reqwest (rustls). Returns the response body; errors on non-2xx or I/O failure.

- **parser.rs**  
  - `Page { text: Vec<String>, links: Vec<(String, String)> }`  
  - `parse(html: &str, base_url: &str) -> Result<Page>`  
  - Selectors: `h1, h2, h3, p` → text (whitespace cleaned); `a[href]` → (anchor text, absolute URL). Skips `#`, `javascript:`, `mailto:`. Relative URLs resolved with `url::Url` and `base_url`.

- **history.rs**  
  - `History { back_stack, forward_stack: Vec<String> }`  
  - `back(current_url)` — pop from back, push current to forward, return the popped URL (or `None`).  
  - `forward(current_url)` — pop from forward, push current to back, return the popped URL (or `None`).

---

### `src/ui/`

TUI only: render from `App`, produce `InputResult` from keys. No fetching or parsing.

- **events.rs**  
  - Polls crossterm key events (100 ms timeout).  
  - Returns `InputResult`: **Quit** (`q`), **Continue**, **FollowLink(url)** (Enter on valid link), **Back** (`b`), **Forward** (`f`).  
  - Updates `app.scroll` (Up/Down) and `app.selected_link` (j/k) in place; does not push history or load pages.

- **layout.rs**  
  - `draw(frame, app)` — builds the ratatui layout:  
    - **URL bar**: `app.current_url`.  
    - **Content**: paragraph from `content_lines(app)` — page text + “--- Links ---” + numbered links; selected link styled (e.g. yellow, reversed); wrap and scroll with `app.scroll`.  
    - **Status bar**: current URL, scroll, link index/count, and control hints.  
  - No logic beyond “turn app state into widgets”; no network or history.

---

## Data flow (summary)

1. **Start**: `App::new()` → `navigate_to(app, "https://example.com")` → fetch + parse → `app.page` and `app.current_url` set.
2. **Loop**:  
   - **Draw**: `ui::draw(frame, &app)` reads `app` and renders URL, content (with scroll and link styles), and status.  
   - **Input**: `ui::handle_input(&mut app)` updates `app.scroll` and `app.selected_link`, returns `InputResult`.  
   - **Act**: main matches on `InputResult` and calls `navigate_to`, `load_page`, or break (quit).
3. **Follow link**: User selects link and presses Enter → `FollowLink(url)` → `navigate_to` pushes current URL to back stack, clears forward stack, fetches and parses `url`, updates `app`.
4. **Back/Forward**: `Back`/`Forward` → `history.back/forward(current_url)` → if `Some(url)`, `load_page(app, url)` (no history push).

---

## Coding standards

- **No `unwrap()`, `expect()`, or `panic!`** in library/app code. Panic hook is only for restoring the terminal on panic.
- **Separation of concerns**:  
  - UI code only renders and maps keys to `InputResult`.  
  - Fetch, parse, and history live in `browser/` or `main`; no logic inside `draw()`.
- **App state drives everything**: All visible state and navigation state come from `App`; no duplicated state in the UI.
- **Small functions**: Prefer small, focused functions; navigation orchestration in `main`, details in `browser/` and `ui/`.
- **Clippy**: Run `cargo clippy` and fix warnings before submitting.

---

## How to contribute

1. **Setup**: Clone the repo, ensure stable Rust (e.g. via `rust-toolchain.toml`). Run `cargo build` and `cargo clippy`.
2. **Changes**: Keep the above architecture: no business logic in `ui/layout.rs`; no TUI or key handling in `browser/`; navigation and history only in `main` and `app`/`browser`.
3. **Testing**: Manually test run: load page, scroll, j/k, Enter, back, forward, quit. Ensure terminal restores correctly after exit and on panic.
4. **PRs**: Prefer small, focused changes. In the PR description, mention which module(s) you touched and how it fits the architecture.

If you want to add features (e.g. bookmarks, tabs, URL bar input), propose the design and where new state and logic will live (prefer extending `App` and `browser/` or `main`) so the “app state drives UI” and “UI only renders” boundaries stay clear.
