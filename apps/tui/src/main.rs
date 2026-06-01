//! {{ project-name }} — a system monitoring TUI.
//!
//! Launch with `{{ project-name }}-tui`. Pass `--demo` for simulated data.

mod app;
mod monitor;
mod theme;
mod ui;

use anyhow::Context;
use std::process::ExitCode;

/// Runs the TUI application.
fn run() -> anyhow::Result<()> {
    // Parse --demo flag (minimal, no clap dependency)
    let demo_mode = std::env::args().any(|a| a == "--demo");

    // Initialize tracing to a file (never stdout during TUI mode)
    let state_dir = dirs::state_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("{{ project-name }}");
    std::fs::create_dir_all(&state_dir)
        .context("failed to create state directory")?;

    let log_file = state_dir.join("app.log");
    let file_appender = tracing_appender::rolling::never(
        state_dir,
        "app.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();

    // Install panic hook to restore terminal before printing
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = ratatui::restore();
        original_hook(info);
    }));

    tracing::info!("Starting {{ project-name }} TUI (demo_mode={demo_mode})");

    // Enter raw mode and alternate screen
    crossterm::terminal::enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)
        .context("failed to enter alternate screen")?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend).context("failed to create terminal")?;

    // Run the app
    let mut app = app::App::new(demo_mode);
    let result = app::run_app(&mut app, &mut terminal);

    // Restore terminal
    let _ = ratatui::restore();
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    );
    let _ = crossterm::terminal::disable_raw_mode();

    tracing::info!("{{ project-name }} TUI exited");
    drop(_guard); // flush tracing

    result
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            // Terminal is restored by now; safe to print to stderr
            eprintln!("Error: {e:#}");
            ExitCode::from(1)
        }
    }
}
