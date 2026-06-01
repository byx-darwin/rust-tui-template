# TUI Patterns

This guide covers the canonical patterns for building terminal user interfaces with `ratatui` and `crossterm`.

## Event Loop

The TUI runs a synchronous event loop on the main thread:

```rust
loop {
    terminal.draw(|f| ui::render(f, &app))?;
    if event::poll(Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => handle_key(&mut app, key),
            Event::Mouse(mouse) => handle_mouse(&mut app, mouse),
            _ => {}
        }
    }
    if app.should_quit { break; }
    // Periodic refresh
    if app.last_refresh.elapsed() >= app.refresh_interval {
        app.refresh_data();
    }
}
```

Use `crossterm::event::poll` with a timeout for responsive input handling. Keep rendering synchronous — ratatui's immediate mode needs no async on the UI thread.

## Terminal Lifecycle

1. `crossterm::terminal::enable_raw_mode()` — disable line buffering and echo
2. `crossterm::execute!(stdout(), EnterAlternateScreen)` — switch to alternate screen buffer
3. `crossterm::execute!(stdout(), EnableMouseCapture)` — enable mouse events
4. Create `ratatui::Terminal::new(CrosstermBackend::new(stdout))`
5. Run the event loop
6. On exit: disable mouse capture, leave alternate screen, disable raw mode

Always use a `TerminalGuard` struct with `Drop` impl, or ensure cleanup in all exit paths (including panics).

## Panic Hook

Always install a custom panic hook that restores the terminal:

```rust
let original_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
    let _ = ratatui::restore();
    original_hook(info);
}));
```

Without this, a panic leaves the user with a broken terminal (no echo, stuck on alternate screen).

## Rendering

ratatui uses immediate-mode rendering. Every frame redraws the entire screen:

- `Layout::vertical` / `Layout::horizontal` with `Constraint` to divide the screen
- Widgets: `Gauge`, `Sparkline`, `Table`, `Paragraph`, `Block`, `Tabs`, `List`
- Stateful widgets need their state passed as `&mut`: `TableState`, `ListState`
- Color via `ratatui::style::Color` (named, `Rgb`, or `Indexed`)

## Input Handling

- `crossterm::event::KeyEvent` for keyboard input
- `crossterm::event::MouseEvent` for mouse clicks and scroll
- Vim-style keybindings (h/j/k/l) alongside arrow keys are the convention
- `Ctrl+C` should trigger a clean exit (same as `q`)

## Async Background Work

For I/O that shouldn't block the UI thread:

1. `tokio::spawn` a background task
2. Send results via `std::sync::mpsc::channel` or `crossbeam::channel`
3. Drain the channel in the event loop: `while let Ok(msg) = rx.try_recv() { ... }`

## Theming

- Define colors in a `theme.rs` module
- Use `Color::Rgb(r, g, b)` for custom colors
- Apply styles via `.fg()`, `.bg()`, `.bold()`, etc.
- Respect `NO_COLOR`: if set, use `Color::Reset` for all styling
- ratatui automatically detects true color support via `$COLORTERM`

## Testing

Library-first approach:

- Test business logic and state transitions in `crates/core` with standard `#[test]`
- Test data collection modules (e.g., `monitor.rs`) by verifying return values
- Widget rendering tests use `ratatui::TestBackend` + `assert_buffer_eq!`
- TUI integration tests require a TTY; skip in CI or use `--demo` mode

## Performance

- Minimize data copies in the render path — cache derived values
- Cap history buffers (e.g., last 60 CPU readings for sparkline)
- Truncate process lists to a reasonable limit (50 entries)
- ratatui diffs the back buffer against the front buffer; you don't need to optimize redraws manually
