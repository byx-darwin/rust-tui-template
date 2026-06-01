//! Frame rendering logic. Called each frame by the event loop.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Sparkline, Table, Tabs, Wrap},
    Frame,
};
use crate::app::{App, Tab};
use crate::i18n::{self, Locale};
use crate::theme::Theme;

/// Main render entry point.
pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    render_tab_bar(frame, areas[0], app);

    match app.active_tab {
        Tab::Overview => render_overview(frame, areas[1], app),
        Tab::Processes => render_processes(frame, areas[1], app),
        Tab::About => render_about(frame, areas[1], app),
    }

    render_status_bar(frame, areas[2], app);

    if app.show_help_bar {
        render_help_bar(frame, areas[3], app);
    }
}

fn render_tab_bar(frame: &mut Frame, area: Rect, app: &App) {
    let locale = app.locale;
    let titles = [
        i18n::tab_overview(locale),
        i18n::tab_processes(locale),
        i18n::tab_about(locale),
    ];
    let tab_lines: Vec<Line> = titles
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let tab = match i {
                0 => Tab::Overview,
                1 => Tab::Processes,
                _ => Tab::About,
            };
            let label = format!(" {t} ");
            if tab == app.active_tab {
                Line::from(label).fg(Theme::TAB_ACTIVE).bold()
            } else {
                Line::from(label).fg(Theme::TAB_INACTIVE)
            }
        })
        .collect();
    let tabs = Tabs::new(tab_lines)
        .block(Block::default().borders(Borders::BOTTOM).border_style(Style::new().fg(Theme::BORDER)))
        .select(match app.active_tab {
            Tab::Overview => 0,
            Tab::Processes => 1,
            Tab::About => 2,
        })
        .divider(symbols::DOT);
    frame.render_widget(tabs, area);
}

fn render_overview(frame: &mut Frame, area: Rect, app: &App) {
    let locale = app.locale;
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(area);

    render_gauge_panel(frame, cols[0], i18n::gauge_cpu(locale), app.snapshot.cpu_usage_pct, &app.snapshot.cpu_history, Theme::gauge_color(app.snapshot.cpu_usage_pct));
    render_gauge_panel(frame, cols[1], i18n::gauge_memory(locale), app.snapshot.memory_usage_pct, &[], Theme::GAUGE_MEMORY);
    render_gauge_panel(frame, cols[2], i18n::gauge_disk(locale), app.snapshot.disk_usage_pct, &[], Theme::GAUGE_DISK);
}

fn render_gauge_panel(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    pct: f64,
    history: &[f64],
    color: ratatui::style::Color,
) {
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Theme::BORDER));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Percentage label
    let label = Paragraph::new(format!("{pct:.1}%"))
        .style(Style::new().fg(color).bold())
        .centered();
    frame.render_widget(label, Rect::new(inner.x, inner.y, inner.width, 2));

    // Gauge
    let gauge = Gauge::default()
        .gauge_style(Style::new().fg(color))
        .percent(pct.round() as u16);
    frame.render_widget(gauge, Rect::new(inner.x + 1, inner.y + 3, inner.width.saturating_sub(2), 1));

    // Sparkline
    if !history.is_empty() {
        let data: Vec<u64> = history.iter().map(|v| (*v * 100.0) as u64).collect();
        let sparkline = Sparkline::default().data(&data).style(Style::new().fg(Theme::SPARKLINE));
        let spark_h = inner.height.saturating_sub(5);
        if spark_h > 0 {
            frame.render_widget(
                sparkline,
                Rect::new(inner.x + 2, inner.y + 5, inner.width.saturating_sub(4), spark_h),
            );
        }
    }
}

fn render_processes(frame: &mut Frame, area: Rect, app: &App) {
    let locale = app.locale;
    let sort_indicator = match app.sort_column {
        crate::app::ProcessSortColumn::Cpu => "CPU ▼",
        crate::app::ProcessSortColumn::Memory => "Mem ▼",
        crate::app::ProcessSortColumn::Name => "Name ▼",
        crate::app::ProcessSortColumn::Pid => "PID ▼",
    };

    let header = Row::new(vec![
        format!("{} {sort_indicator}", i18n::process_header_pid(locale)),
        i18n::process_header_name(locale).to_string(),
        i18n::process_header_cpu(locale).to_string(),
        i18n::process_header_mem(locale).to_string(),
        i18n::process_header_mem_mb(locale).to_string(),
        i18n::process_header_status(locale).to_string(),
    ])
    .style(Style::new().fg(Theme::TABLE_HEADER).bold())
    .bottom_margin(1);

    let rows: Vec<Row> = app
        .snapshot
        .processes
        .iter()
        .map(|p| {
            Row::new(vec![
                p.pid.clone(),
                p.name.clone(),
                format!("{:.1}", p.cpu_usage_pct),
                format!("{:.1}", p.memory_usage_pct),
                format!("{:.1}", p.memory_mb),
                p.status.clone(),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Min(20),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let mut table_state = ratatui::widgets::TableState::default()
        .with_selected(app.process_selected);

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(format!(" {} ({}) ", i18n::tab_processes(locale), app.snapshot.processes.len()))
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Theme::BORDER)),
        )
        .highlight_style(Style::new().bg(Theme::TABLE_SELECTED))
        .row_highlight_style(Style::new().bg(Theme::TABLE_SELECTED));

    frame.render_stateful_widget(table, area, &mut table_state);
}

fn render_about(frame: &mut Frame, area: Rect, app: &App) {
    let locale = app.locale;
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(format!("  {{ project-name }}  [{}]", locale.label()), Style::new().bold())),
        Line::from(format!("  {}", i18n::about_title(locale))),
        Line::from(""),
        Line::from(Span::styled(format!("  {}", i18n::about_section_keybindings(locale)), Style::new().bold().underlined())),
        Line::from(format!("  {}", i18n::about_kb_quit(locale))),
        Line::from(format!("  {}", i18n::about_kb_tab(locale))),
        Line::from(format!("  {}", i18n::about_kb_switch(locale))),
        Line::from(format!("  {}", i18n::about_kb_nav(locale))),
        Line::from(format!("  {}", i18n::about_kb_refresh(locale))),
        Line::from(format!("  {}", i18n::about_kb_interval(locale))),
        Line::from(format!("  {}", i18n::about_kb_sort(locale))),
        Line::from(format!("  {}", i18n::about_kb_lang(locale))),
        Line::from(format!("  {}", i18n::about_kb_help(locale))),
        Line::from(""),
        Line::from(Span::styled(format!("  {}", i18n::about_section_mouse(locale)), Style::new().bold().underlined())),
        Line::from(format!("  {}", i18n::about_mouse_desc(locale))),
    ];
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(format!(" {} ", i18n::tab_about(locale)))
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Theme::BORDER)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let locale = app.locale;
    let bg = Theme::status_bar_bg(app.is_refreshing);
    let status_icon = if app.is_refreshing { "⟳" } else { "●" };
    let status_text = if app.is_refreshing {
        i18n::status_refreshing(locale)
    } else {
        i18n::status_idle(locale)
    };
    let line = Line::from(vec![
        Span::raw(status_icon),
        Span::raw(" "),
        Span::raw(status_text),
        Span::raw("  │  "),
        Span::raw(&app.snapshot.timestamp),
        Span::raw("  │  "),
        Span::raw(i18n::status_refresh_label(locale)),
        Span::raw(": "),
        Span::raw(app.refresh_interval.label()),
        Span::raw("  │  "),
        Span::raw(locale.label()),
    ]);
    let para = Paragraph::new(line).style(Style::new().bg(bg).fg(ratatui::style::Color::White));
    frame.render_widget(para, area);
}

fn render_help_bar(frame: &mut Frame, area: Rect, app: &App) {
    let text = i18n::help_bar_text(app.locale);
    let para = Paragraph::new(text).style(Style::new().fg(Theme::HELP_BAR_FG));
    frame.render_widget(para, area);
}
