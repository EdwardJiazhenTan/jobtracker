use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

/// Render the list view
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
    render_title(frame, chunks[0]);

    // Table
    render_table(frame, app, chunks[1]);

    // Help text
    render_help(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Job Application Tracker")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_table(frame: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["Company", "Platform", "Resume Ver", "Status", "Date"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app.applications.iter().enumerate().map(|(idx, app_record)| {
        let cells = vec![
            Cell::from(app_record.company_name.clone()),
            Cell::from(app_record.platform.as_str()),
            Cell::from(app_record.resume_version.clone()),
            Cell::from(app_record.status.as_str()),
            Cell::from(app_record.applied_date.to_string()),
        ];

        let style = if idx == app.list_selected {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        };

        Row::new(cells).style(style).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Applications"));

    frame.render_widget(table, area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::raw("↑/↓/j/k: Navigate  "),
        Span::styled("a", Style::default().fg(Color::Green)),
        Span::raw(": Add  "),
        Span::styled("e", Style::default().fg(Color::Green)),
        Span::raw(": Edit  "),
        Span::styled("d", Style::default().fg(Color::Green)),
        Span::raw(": Delete  "),
        Span::styled("g", Style::default().fg(Color::Green)),
        Span::raw(": Charts  "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(": Quit"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    frame.render_widget(help, area);
}
