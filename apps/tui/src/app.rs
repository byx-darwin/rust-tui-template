//! Application state and event loop.

use crate::i18n::Locale;
use crate::monitor::SystemSnapshot;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use std::time::{Duration, Instant};

/// Navigation tabs in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// CPU / Memory / Disk gauges.
    Overview,
    /// Process list table.
    Processes,
    /// Keybinding help.
    About,
}

impl Tab {
    /// Cycle to the next tab.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Overview => Self::Processes,
            Self::Processes => Self::About,
            Self::About => Self::Overview,
        }
    }

    /// Cycle to the previous tab.
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Overview => Self::About,
            Self::Processes => Self::Overview,
            Self::About => Self::Processes,
        }
    }
}

/// Refresh interval presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshInterval {
    /// 1 second.
    Fast,
    /// 2 seconds.
    Medium,
    /// 5 seconds.
    Slow,
}

impl RefreshInterval {
    /// Returns the duration for this interval.
    #[must_use]
    pub fn as_duration(self) -> Duration {
        match self {
            Self::Fast => Duration::from_secs(1),
            Self::Medium => Duration::from_secs(2),
            Self::Slow => Duration::from_secs(5),
        }
    }

    /// Human-readable label.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Fast => "1s",
            Self::Medium => "2s",
            Self::Slow => "5s",
        }
    }

    /// Cycle to the next interval.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Fast => Self::Medium,
            Self::Medium => Self::Slow,
            Self::Slow => Self::Fast,
        }
    }
}

/// Sort column for the process table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSortColumn {
    /// Sort by CPU usage.
    Cpu,
    /// Sort by memory usage.
    Memory,
    /// Sort by process name.
    Name,
    /// Sort by process ID.
    Pid,
}

impl ProcessSortColumn {
    /// Cycle to the next sort column.
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Cpu => Self::Memory,
            Self::Memory => Self::Name,
            Self::Name => Self::Pid,
            Self::Pid => Self::Cpu,
        }
    }
}

/// Central application state.
pub struct App {
    /// Currently active tab.
    pub active_tab: Tab,
    /// Scroll offset in the process list.
    pub process_scroll: usize,
    /// Selected row in the process list.
    pub process_selected: Option<usize>,
    /// Current sort column for processes.
    pub sort_column: ProcessSortColumn,
    /// Current refresh interval.
    pub refresh_interval: RefreshInterval,
    /// Timestamp of the most recent data refresh.
    pub last_refresh: Instant,
    /// Whether a refresh is currently in progress.
    pub is_refreshing: bool,
    /// Whether to show the help bar.
    pub show_help_bar: bool,
    /// Whether to use simulated data.
    pub demo_mode: bool,
    /// Current locale for i18n.
    pub locale: Locale,
    /// Latest system data snapshot.
    pub snapshot: SystemSnapshot,
    /// Exit flag.
    pub should_quit: bool,
}

impl App {
    /// Creates a new app with default state.
    #[must_use]
    pub fn new(demo_mode: bool, locale: Locale) -> Self {
        let mut snapshot = SystemSnapshot::new();
        snapshot.refresh(demo_mode);
        Self {
            active_tab: Tab::Overview,
            process_scroll: 0,
            process_selected: Some(0),
            sort_column: ProcessSortColumn::Cpu,
            refresh_interval: RefreshInterval::Fast,
            last_refresh: Instant::now(),
            is_refreshing: false,
            show_help_bar: true,
            demo_mode,
            locale,
            snapshot,
            should_quit: false,
        }
    }
}

/// Handles a single key event.
fn handle_key(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    match (key, modifiers) {
        (KeyCode::Char('c'), m) if m.contains(KeyModifiers::CONTROL) => app.should_quit = true,
        (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => app.should_quit = true,
        (KeyCode::Char('1'), _) => app.active_tab = Tab::Overview,
        (KeyCode::Char('2'), _) => app.active_tab = Tab::Processes,
        (KeyCode::Char('3'), _) => app.active_tab = Tab::About,
        (KeyCode::Left, _) | (KeyCode::Char('h'), _) => app.active_tab = app.active_tab.prev(),
        (KeyCode::Right, _) | (KeyCode::Char('l'), _) => app.active_tab = app.active_tab.next(),
        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => {
            if let Some(sel) = app.process_selected {
                let max = app.snapshot.processes.len().saturating_sub(1);
                app.process_selected = Some((sel + 1).min(max));
            } else if !app.snapshot.processes.is_empty() {
                app.process_selected = Some(0);
            }
        }
        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => {
            if let Some(sel) = app.process_selected {
                app.process_selected = Some(sel.saturating_sub(1));
            }
        }
        (KeyCode::Char('r'), _) => {
            app.snapshot.refresh(app.demo_mode);
            app.last_refresh = Instant::now();
        }
        (KeyCode::Char('f'), _) => app.refresh_interval = app.refresh_interval.next(),
        (KeyCode::Char('s'), _) => app.sort_column = app.sort_column.next(),
        (KeyCode::Char('L'), _) => app.locale = app.locale.next(),
        (KeyCode::Char('?'), _) => app.show_help_bar = !app.show_help_bar,
        _ => {}
    }
}

/// Runs the main event loop.
///
/// # Errors
///
/// Returns an error if terminal rendering or event polling fails.
pub fn run_app(
    app: &mut App,
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| crate::ui::render(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind != KeyEventKind::Release => {
                    handle_key(app, key.code, key.modifiers);
                }
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollDown => {
                        if let Some(sel) = app.process_selected {
                            let max = app.snapshot.processes.len().saturating_sub(1);
                            app.process_selected = Some((sel + 3).min(max));
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if let Some(sel) = app.process_selected {
                            app.process_selected = Some(sel.saturating_sub(3));
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }

        if app.last_refresh.elapsed() >= app.refresh_interval.as_duration() {
            app.is_refreshing = true;
            app.snapshot.refresh(app.demo_mode);
            app.last_refresh = Instant::now();
            app.is_refreshing = false;
        }
    }
    Ok(())
}
