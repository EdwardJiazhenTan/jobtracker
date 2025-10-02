use crate::app::{App, FormField, FormMode};
use crate::models::{Platform, Status};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Render the form view
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(frame.area());

    // Center the form
    let form_area = centered_rect(60, 80, chunks[0]);

    // Title
    let title = match app.form_mode {
        Some(FormMode::Add) => "Add New Application",
        Some(FormMode::Edit(_)) => "Edit Application",
        None => "Form",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(form_area);
    frame.render_widget(block, form_area);

    // Split inner area for fields and help
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(inner_area);

    // Render form fields
    render_fields(frame, app, inner_chunks[0]);

    // Render help
    render_form_help(frame, inner_chunks[1]);
}

fn render_fields(frame: &mut Frame, app: &App, area: Rect) {
    // Adjust constraints based on active field to give dropdowns more space
    let platform_height = if app.form_field == FormField::Platform { 7 } else { 3 };
    let resume_modified_height = if app.form_field == FormField::ResumeModified { 5 } else { 3 };
    let status_height = if app.form_field == FormField::Status { 7 } else { 3 };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Company Name
            Constraint::Length(platform_height), // Platform
            Constraint::Length(resume_modified_height), // Resume Modified
            Constraint::Length(3), // Resume Version
            Constraint::Length(status_height), // Status
            Constraint::Length(3), // Date
            Constraint::Length(5), // Notes (multi-line)
        ])
        .split(area);

    // Company Name
    render_text_field(
        frame,
        chunks[0],
        "Company Name",
        &app.form_data.company_name,
        app.form_field == FormField::CompanyName,
    );

    // Platform
    if app.form_field == FormField::Platform {
        render_dropdown_field(
            frame,
            chunks[1],
            "Platform",
            Platform::presets(),
            app.platform_dropdown_selected,
        );
    } else {
        render_text_field(
            frame,
            chunks[1],
            "Platform",
            &app.form_data.platform.as_str(),
            false,
        );
    }

    // Resume Modified
    if app.form_field == FormField::ResumeModified {
        render_dropdown_field(
            frame,
            chunks[2],
            "Resume Modified",
            &["Yes", "No"],
            app.resume_modified_dropdown_selected,
        );
    } else {
        render_text_field(
            frame,
            chunks[2],
            "Resume Modified",
            if app.form_data.resume_modified { "Yes" } else { "No" },
            false,
        );
    }

    // Resume Version
    render_text_field(
        frame,
        chunks[3],
        "Resume Version",
        &app.form_data.resume_version,
        app.form_field == FormField::ResumeVersion,
    );

    // Status
    if app.form_field == FormField::Status {
        let status_options: Vec<&str> = Status::all().iter().map(|s| s.as_str()).collect();
        render_dropdown_field(
            frame,
            chunks[4],
            "Status",
            &status_options,
            app.status_dropdown_selected,
        );
    } else {
        render_text_field(
            frame,
            chunks[4],
            "Status",
            app.form_data.status.as_str(),
            false,
        );
    }

    // Date
    render_text_field(
        frame,
        chunks[5],
        "Application Date",
        &app.form_data.applied_date.to_string(),
        app.form_field == FormField::Date,
    );

    // Notes
    render_text_field(
        frame,
        chunks[6],
        "Notes",
        &app.form_data.notes,
        app.form_field == FormField::Notes,
    );
}

fn render_text_field(frame: &mut Frame, area: Rect, label: &str, value: &str, is_selected: bool) {
    let style = if is_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let text = vec![
        Line::from(vec![
            Span::styled(format!("{}: ", label), style),
            Span::raw(value),
        ]),
    ];

    let block = Block::default().borders(Borders::NONE);
    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_dropdown_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    options: &[&str],
    selected: usize,
) {
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(idx, opt)| {
            let style = if idx == selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(*opt).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(format!("{} (j/k to select)", label))
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(list, area);
}

fn render_form_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("↑/↓", Style::default().fg(Color::Green)),
        Span::raw(": Navigate Fields  "),
        Span::styled("j/k", Style::default().fg(Color::Green)),
        Span::raw(": Select in Dropdown  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(": Next/Save  "),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::raw(": Cancel"),
    ];

    let help = Paragraph::new(Line::from(help_text))
        .alignment(Alignment::Center);
    frame.render_widget(help, area);
}

/// Create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
