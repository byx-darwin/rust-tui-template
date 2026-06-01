//! Internationalization via compile-time match statements.
//! Zero external dependencies. Add languages by adding match arms.

/// Supported locales.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    /// English (default).
    En,
    /// Simplified Chinese.
    Zh,
}

impl Locale {
    /// Detect locale from `LANG` environment variable.
    #[must_use]
    pub fn from_env() -> Self {
        match std::env::var("LANG").unwrap_or_default().as_str() {
            s if s.starts_with("zh") => Self::Zh,
            _ => Self::En,
        }
    }

    /// Human-readable label for the locale.  }  }
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::Zh => "中文",
        }
    }

    /// Cycle to the next locale.  }  }
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::En => Self::Zh,
            Self::Zh => Self::En,
        }
    }
}

// ── Tab labels ──────────────────────────────────────────────

pub fn tab_overview(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Overview",
        Locale::Zh => "概览",
    }
}

pub fn tab_processes(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Processes",
        Locale::Zh => "进程",
    }
}

pub fn tab_about(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "About",
        Locale::Zh => "关于",
    }
}

// ── Gauge titles ────────────────────────────────────────────

pub fn gauge_cpu(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "CPU",
        Locale::Zh => "CPU",
    }
}

pub fn gauge_memory(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Memory",
        Locale::Zh => "内存",
    }
}

pub fn gauge_disk(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Disk",
        Locale::Zh => "磁盘",
    }
}

// ── Process table ───────────────────────────────────────────

pub fn process_header_pid(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "PID",
        Locale::Zh => "PID",
    }
}

pub fn process_header_name(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Name",
        Locale::Zh => "名称",
    }
}

pub fn process_header_cpu(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "CPU%",
        Locale::Zh => "CPU%",
    }
}

pub fn process_header_mem(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Mem%",
        Locale::Zh => "内存%",
    }
}

pub fn process_header_mem_mb(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Mem MB",
        Locale::Zh => "内存 MB",
    }
}

pub fn process_header_status(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Status",
        Locale::Zh => "状态",
    }
}

// ── Status bar ──────────────────────────────────────────────

pub fn status_idle(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Idle",
        Locale::Zh => "空闲",
    }
}

pub fn status_refreshing(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Refreshing",
        Locale::Zh => "刷新中",
    }
}

pub fn status_refresh_label(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Refresh",
        Locale::Zh => "刷新",
    }
}

// ── Help bar ────────────────────────────────────────────────

pub fn help_bar_text(locale: Locale) -> &'static str {
    match locale {
        Locale::En => " q:Quit  1-3:Tab  ←→/hl:Switch  ↑↓/jk:Nav  r:Refresh  f:Interval  s:Sort  L:Lang  ?:Help ",
        Locale::Zh => " q:退出  1-3:标签  ←→/hl:切换  ↑↓/jk:导航  r:刷新  f:间隔  s:排序  L:语言  ?:帮助 ",
    }
}

// ── About tab ───────────────────────────────────────────────

pub fn about_title(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "A system monitoring TUI",
        Locale::Zh => "系统监控 TUI",
    }
}

pub fn about_section_keybindings(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Keybindings",
        Locale::Zh => "快捷键",
    }
}

pub fn about_section_mouse(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Mouse",
        Locale::Zh => "鼠标",
    }
}

pub fn about_mouse_desc(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "Scroll wheel to browse the process list.",
        Locale::Zh => "滚轮浏览进程列表。",
    }
}

pub fn about_kb_quit(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "q / Esc       Quit",
        Locale::Zh => "q / Esc       退出",
    }
}

pub fn about_kb_tab(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "1 2 3         Switch to tab",
        Locale::Zh => "1 2 3         切换标签",
    }
}

pub fn about_kb_switch(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "h l / ← →     Previous / next tab",
        Locale::Zh => "h l / ← →     前一个/后一个标签",
    }
}

pub fn about_kb_nav(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "j k / ↑ ↓     Navigate / scroll",
        Locale::Zh => "j k / ↑ ↓     导航/滚动",
    }
}

pub fn about_kb_refresh(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "r             Manual refresh",
        Locale::Zh => "r             手动刷新",
    }
}

pub fn about_kb_interval(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "f             Cycle refresh interval (1s → 2s → 5s)",
        Locale::Zh => "f             循环刷新间隔 (1秒 → 2秒 → 5秒)",
    }
}

pub fn about_kb_sort(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "s             Cycle process sort column",
        Locale::Zh => "s             循环排序 (CPU → 内存 → 名称 → PID)",
    }
}

pub fn about_kb_lang(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "L             Switch language",
        Locale::Zh => "L             切换语言",
    }
}

pub fn about_kb_help(locale: Locale) -> &'static str {
    match locale {
        Locale::En => "?             Toggle help bar",
        Locale::Zh => "?             切换帮助栏",
    }
}
