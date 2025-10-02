use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Application status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Applied,
    Interview,
    Offer,
    Rejected,
}

impl Status {
    pub fn all() -> &'static [Status] {
        &[Status::Applied, Status::Interview, Status::Offer, Status::Rejected]
    }

    pub fn as_str(&self) -> &str {
        match self {
            Status::Applied => "Applied",
            Status::Interview => "Interview",
            Status::Offer => "Offer",
            Status::Rejected => "Rejected",
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Applied
    }
}

/// Platform enum with common presets
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    LinkedIn,
    Indeed,
    CompanyWebsite,
    Other(String),
}

impl Platform {
    pub fn presets() -> &'static [&'static str] {
        &["LinkedIn", "Indeed", "Company Website", "Other"]
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "LinkedIn" => Platform::LinkedIn,
            "Indeed" => Platform::Indeed,
            "Company Website" => Platform::CompanyWebsite,
            _ => Platform::Other(s.to_string()),
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Platform::LinkedIn => "LinkedIn".to_string(),
            Platform::Indeed => "Indeed".to_string(),
            Platform::CompanyWebsite => "Company Website".to_string(),
            Platform::Other(s) => s.clone(),
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        Platform::LinkedIn
    }
}

/// Job application record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub company_name: String,
    pub platform: Platform,
    pub resume_modified: bool,
    pub resume_version: String,
    pub status: Status,
    pub applied_date: NaiveDate,
    pub notes: String,
}

impl Application {
    pub fn new() -> Self {
        Self {
            company_name: String::new(),
            platform: Platform::default(),
            resume_modified: false,
            resume_version: String::new(),
            status: Status::default(),
            applied_date: chrono::Local::now().date_naive(),
            notes: String::new(),
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
