use anyhow::Result;
use crossterm::event::KeyCode;

use crate::{app::App, handlers::navigation::NavigationHelper};

/// Common functionality shared across different modes
pub struct CommonModeLogic;

impl CommonModeLogic {
    /// Handle common exit functionality (Esc key in normal mode, Enter for selection)
    pub fn handle_exit_keys(app: &mut App, key: KeyCode) -> Result<Option<bool>> {
        match key {
            KeyCode::Esc => {
                // In normal mode, Esc exits the application
                if matches!(app.state.mode, crate::models::AppMode::Normal) {
                    return Ok(Some(false));
                }
                // In other modes, Esc returns to normal mode
                app.enter_normal_mode();
                Ok(Some(true))
            }
            KeyCode::Enter => {
                // Handle selection and exit
                let selected_file = app.get_selected_file().cloned();
                crate::events::handle_exit(app, selected_file.as_ref())?;
                Ok(Some(true))
            }
            _ => Ok(None),
        }
    }
    
    /// Handle common navigation for file-based modes (Normal/Search)
    pub fn handle_file_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('k') | KeyCode::Up => {
                Ok(NavigationHelper::navigate_file_list_up(app))
            }
            KeyCode::Char('j') | KeyCode::Down => {
                Ok(NavigationHelper::navigate_file_list_down(app))
            }
            KeyCode::Char('l') | KeyCode::Right => {
                NavigationHelper::navigate_into_directory(app)
            }
            KeyCode::Char('h') | KeyCode::Left => {
                NavigationHelper::navigate_to_parent(app)
            }
            _ => Ok(false),
        }
    }
    
    /// Handle common navigation for history mode
    pub fn handle_history_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('k') | KeyCode::Up => {
                Ok(NavigationHelper::navigate_history_up(app))
            }
            KeyCode::Char('j') | KeyCode::Down => {
                Ok(NavigationHelper::navigate_history_down(app))
            }
            _ => Ok(false),
        }
    }
    
    /// Handle mode switching keys
    pub fn handle_mode_switches(app: &mut App, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('/') => {
                app.enter_search_mode();
                Ok(true)
            }
            KeyCode::Char('v') => {
                app.enter_history_mode();
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
