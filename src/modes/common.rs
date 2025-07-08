use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::{app::App, handlers::navigation::NavigationHelper, modes::ModeAction};

/// Common functionality shared across different modes
pub struct CommonModeLogic;

impl CommonModeLogic {
    /// Handle common exit functionality (Esc key in normal mode, Enter for selection)
    pub fn handle_exit_keys(
        app: &mut App,
        key: KeyCode,
        current_mode: &crate::models::AppMode,
    ) -> Result<Option<ModeAction>> {
        match key {
            KeyCode::Esc => {
                // In normal mode, Esc exits the application
                if current_mode == &crate::models::AppMode::Normal {
                    return Ok(Some(ModeAction::Exit(None)));
                }
                // In other modes, Esc returns to normal mode
                Ok(Some(ModeAction::Switch(crate::models::AppMode::Normal)))
            }
            KeyCode::Enter => {
                // Handle selection and exit
                let selected_file = app.get_selected_file().cloned();
                Ok(Some(ModeAction::Exit(selected_file)))
            }
            _ => Ok(None),
        }
    }

    /// Handle common navigation for file-based modes (Normal/Search)
    pub fn handle_file_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('k') | KeyCode::Up => Ok(NavigationHelper::navigate_file_list_up(app)),
            KeyCode::Char('j') | KeyCode::Down => {
                Ok(NavigationHelper::navigate_file_list_down(app))
            }
            KeyCode::Char('l') | KeyCode::Right => NavigationHelper::navigate_into_directory(app),
            KeyCode::Char('h') | KeyCode::Left => NavigationHelper::navigate_to_parent(app),
            _ => Ok(false),
        }
    }

    /// Handle common navigation for history mode
    pub fn handle_history_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('k') | KeyCode::Up => Ok(NavigationHelper::navigate_history_up(app)),
            KeyCode::Char('j') | KeyCode::Down => Ok(NavigationHelper::navigate_history_down(app)),
            _ => Ok(false),
        }
    }

    /// Handle mode switching keys
    pub fn handle_mode_switches(_app: &mut App, key: KeyCode) -> Result<Option<ModeAction>> {
        match key {
            KeyCode::Char('/') => Ok(Some(ModeAction::Switch(crate::models::AppMode::Search))),
            KeyCode::Char('v') => Ok(Some(ModeAction::Switch(crate::models::AppMode::History))),
            _ => Ok(None),
        }
    }

    /// Handle mouse scroll navigation
    pub fn handle_scroll_navigation(app: &mut App, mouse: MouseEvent) -> Result<bool> {
        match mouse.kind {
            MouseEventKind::ScrollUp => Ok(NavigationHelper::navigate_file_list_up(app)),
            MouseEventKind::ScrollDown => Ok(NavigationHelper::navigate_file_list_down(app)),
            _ => Ok(false),
        }
    }

    /// Handle file list mouse click navigation
    pub fn handle_file_list_mouse_click(app: &mut App, mouse: MouseEvent, area: Rect) -> Result<bool> {
        match mouse.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                // Calculate which file was clicked based on mouse position
                if mouse.row >= area.y && mouse.row < area.y + area.height {
                    let clicked_index = (mouse.row - area.y) as usize;
                    if clicked_index < app.state.filtered_files.len() {
                        app.state.file_list_state.select(Some(clicked_index));
                        // If it's a directory, navigate into it
                        if let Some(file) = app.get_selected_file() {
                            if file.is_dir {
                                return NavigationHelper::navigate_into_directory(app);
                            }
                        }
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Handle position-aware mouse scroll navigation
    pub fn handle_position_aware_scroll_navigation(
        app: &mut App, 
        mouse: MouseEvent, 
        left_area: Rect, 
        right_area: Rect
    ) -> Result<bool> {
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                // Check if mouse is in left area (file list) or right area (preview)
                if mouse.column >= left_area.x && mouse.column < left_area.x + left_area.width {
                    // Mouse is in left panel - scroll file list
                    Ok(NavigationHelper::navigate_file_list_up(app))
                } else if mouse.column >= right_area.x && mouse.column < right_area.x + right_area.width {
                    // Mouse is in right panel - scroll preview content
                    Ok(app.scroll_preview_up())
                } else {
                    Ok(false)
                }
            }
            MouseEventKind::ScrollDown => {
                // Check if mouse is in left area (file list) or right area (preview)
                if mouse.column >= left_area.x && mouse.column < left_area.x + left_area.width {
                    // Mouse is in left panel - scroll file list
                    Ok(NavigationHelper::navigate_file_list_down(app))
                } else if mouse.column >= right_area.x && mouse.column < right_area.x + right_area.width {
                    // Mouse is in right panel - scroll preview content
                    Ok(app.scroll_preview_down())
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}
