//! Theme constants for the TUI system monitor.

use ratatui::style::{Color, Style, Modifier};

/// Color theme for the system monitoring panel.
#[allow(dead_code, reason = "Theme is consumed by ui.rs; all fields are used at render time")]
pub struct Theme;

impl Theme {
    pub const TAB_ACTIVE: Color = Color::Yellow;
    pub const TAB_INACTIVE: Color = Color::DarkGray;
    pub const GAUGE_CPU_GREEN: Color = Color::Rgb(0, 200, 80);
    pub const GAUGE_CPU_YELLOW: Color = Color::Rgb(255, 200, 0);
    pub const GAUGE_CPU_RED: Color = Color::Rgb(220, 50, 50);
    pub const GAUGE_MEMORY: Color = Color::Rgb(70, 130, 200);
    pub const GAUGE_DISK: Color = Color::Rgb(160, 100, 220);
    pub const SPARKLINE: Color = Color::Rgb(0, 180, 220);
    pub const TABLE_HEADER: Color = Color::Yellow;
    pub const TABLE_SELECTED: Color = Color::DarkGray;
    pub const STATUS_BAR_BG: Color = Color::DarkGray;
    pub const STATUS_BAR_REFRESHING: Color = Color::Rgb(80, 60, 0);
    pub const HELP_BAR_FG: Color = Color::DarkGray;
    pub const BORDER: Color = Color::DarkGray;
    pub const TEXT_DIM: Color = Color::DarkGray;

    /// Returns the gauge color based on a percentage value.
    #[must_use]
    pub fn gauge_color(pct: f64) -> Color {
        if pct >= 90.0 {
            Self::GAUGE_CPU_RED
        } else if pct >= 70.0 {
            Self::GAUGE_CPU_YELLOW
        } else {
            Self::GAUGE_CPU_GREEN
        }
    }

    /// Returns the status bar background based on refresh state.
    #[must_use]
    pub fn status_bar_bg(is_refreshing: bool) -> Color {
        if is_refreshing {
            Self::STATUS_BAR_REFRESHING
        } else {
            Self::STATUS_BAR_BG
        }
    }
}
