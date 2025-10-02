use crate::models::{Application, Platform, Status};
use crate::storage;
use anyhow::Result;

/// Current view/screen in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    List,
    Form,
    Chart,
}

/// Form mode: adding new or editing existing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormMode {
    Add,
    Edit(usize),
}

/// Form field being edited
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormField {
    CompanyName,
    Platform,
    ResumeModified,
    ResumeVersion,
    Status,
    Date,
    Notes,
}

impl FormField {
    pub fn all() -> &'static [FormField] {
        &[
            FormField::CompanyName,
            FormField::Platform,
            FormField::ResumeModified,
            FormField::ResumeVersion,
            FormField::Status,
            FormField::Date,
            FormField::Notes,
        ]
    }

    pub fn next(&self) -> Self {
        let fields = Self::all();
        let current_idx = fields.iter().position(|f| f == self).unwrap();
        fields[(current_idx + 1) % fields.len()]
    }

    pub fn prev(&self) -> Self {
        let fields = Self::all();
        let current_idx = fields.iter().position(|f| f == self).unwrap();
        if current_idx == 0 {
            fields[fields.len() - 1]
        } else {
            fields[current_idx - 1]
        }
    }

    pub fn label(&self) -> &str {
        match self {
            FormField::CompanyName => "Company Name",
            FormField::Platform => "Platform",
            FormField::ResumeModified => "Resume Modified",
            FormField::ResumeVersion => "Resume Version",
            FormField::Status => "Status",
            FormField::Date => "Application Date",
            FormField::Notes => "Notes",
        }
    }
}

/// Chart type for statistics view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    ByResumeVersion,
    ByPlatform,
    ByStatus,
}

impl ChartType {
    pub fn all() -> &'static [ChartType] {
        &[
            ChartType::ByResumeVersion,
            ChartType::ByPlatform,
            ChartType::ByStatus,
        ]
    }

    pub fn next(&self) -> Self {
        let charts = Self::all();
        let current_idx = charts.iter().position(|c| c == self).unwrap();
        charts[(current_idx + 1) % charts.len()]
    }

    pub fn title(&self) -> &str {
        match self {
            ChartType::ByResumeVersion => "Applications by Resume Version",
            ChartType::ByPlatform => "Applications by Platform",
            ChartType::ByStatus => "Applications by Status",
        }
    }
}

/// Main application state
pub struct App {
    pub applications: Vec<Application>,
    pub view: View,
    pub list_selected: usize,
    pub form_mode: Option<FormMode>,
    pub form_field: FormField,
    pub form_data: Application,
    pub platform_dropdown_selected: usize,
    pub status_dropdown_selected: usize,
    pub resume_modified_dropdown_selected: usize,
    pub chart_type: ChartType,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let applications = storage::load_applications()?;
        Ok(Self {
            applications,
            view: View::List,
            list_selected: 0,
            form_mode: None,
            form_field: FormField::CompanyName,
            form_data: Application::new(),
            platform_dropdown_selected: 0,
            status_dropdown_selected: 0,
            resume_modified_dropdown_selected: 0,
            chart_type: ChartType::ByResumeVersion,
            should_quit: false,
        })
    }

    /// Save applications to disk
    pub fn save(&self) -> Result<()> {
        storage::save_applications(&self.applications)
    }

    /// Start adding a new application
    pub fn start_add(&mut self) {
        self.form_mode = Some(FormMode::Add);
        self.view = View::Form;
        self.form_field = FormField::CompanyName;
        self.form_data = Application::new();
        self.platform_dropdown_selected = 0;
        self.status_dropdown_selected = 0;
        self.resume_modified_dropdown_selected = 0;
    }

    /// Start editing the selected application
    pub fn start_edit(&mut self) {
        if self.applications.is_empty() {
            return;
        }

        let index = self.list_selected;
        self.form_mode = Some(FormMode::Edit(index));
        self.view = View::Form;
        self.form_field = FormField::CompanyName;
        self.form_data = self.applications[index].clone();

        // Set dropdown selections to match current values
        self.status_dropdown_selected = Status::all()
            .iter()
            .position(|s| *s == self.form_data.status)
            .unwrap_or(0);

        // Platform dropdown selection
        let platform_str = self.form_data.platform.as_str();
        self.platform_dropdown_selected = Platform::presets()
            .iter()
            .position(|&p| {
                if p == "Other" {
                    !matches!(self.form_data.platform, Platform::LinkedIn | Platform::Indeed | Platform::CompanyWebsite)
                } else {
                    Platform::from_str(p).as_str() == platform_str
                }
            })
            .unwrap_or(0);

        // Resume modified dropdown selection
        self.resume_modified_dropdown_selected = if self.form_data.resume_modified { 0 } else { 1 };
    }

    /// Save the form data
    pub fn save_form(&mut self) -> Result<()> {
        // Validate
        if self.form_data.company_name.trim().is_empty() {
            return Ok(()); // Silent validation - don't save if company name is empty
        }

        match self.form_mode {
            Some(FormMode::Add) => {
                self.applications.push(self.form_data.clone());
            }
            Some(FormMode::Edit(index)) => {
                self.applications[index] = self.form_data.clone();
            }
            None => {}
        }

        self.save()?;
        self.view = View::List;
        self.form_mode = None;

        Ok(())
    }

    /// Cancel form editing
    pub fn cancel_form(&mut self) {
        self.view = View::List;
        self.form_mode = None;
    }

    /// Delete the selected application
    pub fn delete_selected(&mut self) -> Result<()> {
        if !self.applications.is_empty() {
            self.applications.remove(self.list_selected);
            if self.list_selected >= self.applications.len() && self.list_selected > 0 {
                self.list_selected -= 1;
            }
            self.save()?;
        }
        Ok(())
    }

    /// Move list selection up
    pub fn select_previous(&mut self) {
        if !self.applications.is_empty() {
            if self.list_selected > 0 {
                self.list_selected -= 1;
            }
        }
    }

    /// Move list selection down
    pub fn select_next(&mut self) {
        if !self.applications.is_empty() {
            if self.list_selected < self.applications.len() - 1 {
                self.list_selected += 1;
            }
        }
    }

    /// Switch to chart view
    pub fn show_chart(&mut self) {
        self.view = View::Chart;
        self.chart_type = ChartType::ByResumeVersion;
    }

    /// Switch to next chart type
    pub fn next_chart(&mut self) {
        self.chart_type = self.chart_type.next();
    }

    /// Return to list view
    pub fn show_list(&mut self) {
        self.view = View::List;
    }

    /// Move to next form field
    pub fn next_field(&mut self) {
        self.form_field = self.form_field.next();
    }

    /// Move to previous form field
    pub fn prev_field(&mut self) {
        self.form_field = self.form_field.prev();
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
