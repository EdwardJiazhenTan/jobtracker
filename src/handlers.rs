use crate::app::{App, FormField, View};
use crate::models::{Platform, Status};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle keyboard events based on current view
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    match app.view {
        View::List => handle_list_keys(app, key),
        View::Form => handle_form_keys(app, key),
        View::Chart => handle_chart_keys(app, key),
    }
}

/// Handle keyboard events in list view
fn handle_list_keys(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('a') => app.start_add(),
        KeyCode::Char('e') => app.start_edit(),
        KeyCode::Char('d') => app.delete_selected()?,
        KeyCode::Char('g') => app.show_chart(),
        KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
        _ => {}
    }
    Ok(())
}

/// Handle keyboard events in form view
fn handle_form_keys(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => app.cancel_form(),
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.save_form()?;
        }
        KeyCode::Enter => {
            // In dropdown fields, Enter selects the option and moves to next field
            // On the last field (Notes), Enter saves the form
            match app.form_field {
                FormField::Platform => {
                    let selected = Platform::presets()[app.platform_dropdown_selected];
                    app.form_data.platform = Platform::from_str(selected);
                    app.next_field();
                }
                FormField::Status => {
                    app.form_data.status = Status::all()[app.status_dropdown_selected];
                    app.next_field();
                }
                FormField::ResumeModified => {
                    app.form_data.resume_modified = app.resume_modified_dropdown_selected == 0;
                    app.next_field();
                }
                FormField::Notes => {
                    // Last field - save the form
                    app.save_form()?;
                }
                _ => app.next_field(),
            }
        }
        KeyCode::Up => {
            // Up arrow moves to previous field
            app.prev_field();
        }
        KeyCode::Down => {
            // Down arrow moves to next field
            app.next_field();
        }
        KeyCode::Char('k') => {
            // k navigates within dropdown options
            match app.form_field {
                FormField::Platform => {
                    if app.platform_dropdown_selected > 0 {
                        app.platform_dropdown_selected -= 1;
                    }
                }
                FormField::Status => {
                    if app.status_dropdown_selected > 0 {
                        app.status_dropdown_selected -= 1;
                    }
                }
                FormField::ResumeModified => {
                    if app.resume_modified_dropdown_selected > 0 {
                        app.resume_modified_dropdown_selected -= 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Char('j') => {
            // j navigates within dropdown options
            match app.form_field {
                FormField::Platform => {
                    if app.platform_dropdown_selected < Platform::presets().len() - 1 {
                        app.platform_dropdown_selected += 1;
                    }
                }
                FormField::Status => {
                    if app.status_dropdown_selected < Status::all().len() - 1 {
                        app.status_dropdown_selected += 1;
                    }
                }
                FormField::ResumeModified => {
                    if app.resume_modified_dropdown_selected < 1 {
                        app.resume_modified_dropdown_selected += 1;
                    }
                }
                _ => {}
            }
        }
        KeyCode::Char(c) => {
            handle_text_input(app, c);
        }
        KeyCode::Backspace => {
            handle_backspace(app);
        }
        _ => {}
    }
    Ok(())
}

/// Handle keyboard events in chart view
fn handle_chart_keys(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => app.show_list(),
        KeyCode::Tab => app.next_chart(),
        _ => {}
    }
    Ok(())
}

/// Handle text input for form fields
fn handle_text_input(app: &mut App, c: char) {
    // Skip j/k for dropdown navigation
    if c == 'j' || c == 'k' {
        return;
    }

    match app.form_field {
        FormField::CompanyName => app.form_data.company_name.push(c),
        FormField::ResumeVersion => app.form_data.resume_version.push(c),
        FormField::Notes => app.form_data.notes.push(c),
        FormField::Platform => {
            // If on "Other" option, allow custom input
            if app.platform_dropdown_selected == Platform::presets().len() - 1 {
                if let Platform::Other(ref mut custom) = app.form_data.platform {
                    custom.push(c);
                } else {
                    app.form_data.platform = Platform::Other(c.to_string());
                }
            }
        }
        FormField::Date => {
            // Allow date input (basic implementation)
            // Format: YYYY-MM-DD
            let current = app.form_data.applied_date.to_string();
            if c.is_ascii_digit() || c == '-' {
                let new_date = format!("{}{}", current, c);
                if let Ok(date) = chrono::NaiveDate::parse_from_str(&new_date, "%Y-%m-%d") {
                    app.form_data.applied_date = date;
                }
            }
        }
        _ => {}
    }
}

/// Handle backspace for form fields
fn handle_backspace(app: &mut App) {
    match app.form_field {
        FormField::CompanyName => {
            app.form_data.company_name.pop();
        }
        FormField::ResumeVersion => {
            app.form_data.resume_version.pop();
        }
        FormField::Notes => {
            app.form_data.notes.pop();
        }
        FormField::Platform => {
            // If on "Other" option, allow backspace
            if app.platform_dropdown_selected == Platform::presets().len() - 1 {
                if let Platform::Other(ref mut custom) = app.form_data.platform {
                    custom.pop();
                }
            }
        }
        _ => {}
    }
}
