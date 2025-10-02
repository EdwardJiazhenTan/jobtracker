pub mod list;
pub mod form;
pub mod chart;

use crate::app::{App, View};
use ratatui::{
    backend::Backend,
    Terminal,
};
use anyhow::Result;

/// Main UI rendering function
pub fn render<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<()> {
    terminal.draw(|frame| {
        match app.view {
            View::List => list::render(frame, app),
            View::Form => form::render(frame, app),
            View::Chart => chart::render(frame, app),
        }
    })?;
    Ok(())
}
