use crate::models::Application;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "applications.json";

/// Load applications from JSON file
pub fn load_applications() -> Result<Vec<Application>> {
    let path = Path::new(DATA_FILE);

    if !path.exists() {
        // Return empty vector if file doesn't exist
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)
        .context("Failed to read applications file")?;

    let applications: Vec<Application> = serde_json::from_str(&content)
        .context("Failed to parse applications JSON")?;

    Ok(applications)
}

/// Save applications to JSON file
pub fn save_applications(applications: &[Application]) -> Result<()> {
    let json = serde_json::to_string_pretty(applications)
        .context("Failed to serialize applications")?;

    fs::write(DATA_FILE, json)
        .context("Failed to write applications file")?;

    Ok(())
}
