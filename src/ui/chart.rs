use crate::app::{App, ChartType};
use crate::models::Status;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
    Frame,
};
use std::collections::HashMap;

/// Render the chart view
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new(app.chart_type.title())
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Chart
    render_chart(frame, app, chunks[1]);

    // Help
    render_chart_help(frame, chunks[2]);
}

fn render_chart(frame: &mut Frame, app: &App, area: Rect) {
    match app.chart_type {
        ChartType::ByResumeVersion => render_resume_version_chart(frame, app, area),
        ChartType::ByPlatform => render_platform_chart(frame, app, area),
        ChartType::ByStatus => render_status_chart(frame, app, area),
    }
}

fn render_resume_version_chart(frame: &mut Frame, app: &App, area: Rect) {
    let mut counts: HashMap<String, u64> = HashMap::new();

    for application in &app.applications {
        let version = if application.resume_version.is_empty() {
            "None".to_string()
        } else {
            application.resume_version.clone()
        };
        *counts.entry(version).or_insert(0) += 1;
    }

    let mut data: Vec<(String, u64)> = counts.into_iter().collect();
    // Sort by count descending, then by name ascending for stable sort
    data.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    if data.is_empty() {
        let empty = Paragraph::new("No data available")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(empty, area);
        return;
    }

    // Take top 10
    data.truncate(10);

    let bars: Vec<Bar> = data
        .iter()
        .map(|(label, count)| {
            Bar::default()
                .value(*count)
                .label(Line::from(label.as_str()))
                .style(Style::default().fg(Color::Green))
        })
        .collect();

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Count by Resume Version"))
        .data(BarGroup::default().bars(&bars))
        .bar_width(9)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Green));

    frame.render_widget(chart, area);
}

fn render_platform_chart(frame: &mut Frame, app: &App, area: Rect) {
    let mut counts: HashMap<String, u64> = HashMap::new();

    for application in &app.applications {
        let platform = application.platform.as_str();
        *counts.entry(platform).or_insert(0) += 1;
    }

    let mut data: Vec<(String, u64)> = counts.into_iter().collect();
    // Sort by count descending, then by name ascending for stable sort
    data.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    if data.is_empty() {
        let empty = Paragraph::new("No data available")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(empty, area);
        return;
    }

    let bars: Vec<Bar> = data
        .iter()
        .map(|(label, count)| {
            Bar::default()
                .value(*count)
                .label(Line::from(label.as_str()))
                .style(Style::default().fg(Color::Blue))
        })
        .collect();

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Count by Platform"))
        .data(BarGroup::default().bars(&bars))
        .bar_width(9)
        .bar_gap(1)
        .bar_style(Style::default().fg(Color::Blue));

    frame.render_widget(chart, area);
}

fn render_status_chart(frame: &mut Frame, app: &App, area: Rect) {
    let mut counts: HashMap<String, u64> = HashMap::new();

    for application in &app.applications {
        let status = application.status.as_str();
        *counts.entry(status.to_string()).or_insert(0) += 1;
    }

    // Ensure all statuses are represented
    for status in Status::all() {
        counts.entry(status.as_str().to_string()).or_insert(0);
    }

    let data: Vec<(String, u64)> = Status::all()
        .iter()
        .map(|s| (s.as_str().to_string(), *counts.get(s.as_str()).unwrap_or(&0)))
        .collect();

    if data.iter().all(|(_, count)| *count == 0) {
        let empty = Paragraph::new("No data available")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(empty, area);
        return;
    }

    let bars: Vec<Bar> = data
        .iter()
        .map(|(label, count)| {
            let color = match label.as_str() {
                "Applied" => Color::Yellow,
                "Interview" => Color::Cyan,
                "Offer" => Color::Green,
                "Rejected" => Color::Red,
                _ => Color::White,
            };

            Bar::default()
                .value(*count)
                .label(Line::from(label.as_str()))
                .style(Style::default().fg(color))
        })
        .collect();

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Count by Status"))
        .data(BarGroup::default().bars(&bars))
        .bar_width(9)
        .bar_gap(1);

    frame.render_widget(chart, area);
}

fn render_chart_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("Tab", Style::default().fg(Color::Green)),
        Span::raw(": Switch Chart  "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(": Back to List"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(help, area);
}
