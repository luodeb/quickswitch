use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use std::time::Instant;

use crate::{app::App, handlers::navigation::NavigationHelper, modes::ModeAction};

/// Double click detection interval in milliseconds
const DOUBLE_CLICK_INTERVAL_MS: u64 = 150;

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
            KeyCode::PageUp | KeyCode::PageDown => Self::handle_preview_navigation(app, key),
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
    pub fn handle_file_list_mouse_click(
        app: &mut App,
        mouse: MouseEvent,
        area: Rect,
    ) -> Result<bool> {
        match mouse.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                // Calculate which file was clicked based on mouse position
                if mouse.row >= area.y && mouse.row < area.y + area.height {
                    let visible_clicked_index = (mouse.row - area.y - 1) as usize;
                    let scroll_offset = app.state.file_list_state.offset();
                    let clicked_index = visible_clicked_index + scroll_offset;
                    if clicked_index < app.state.filtered_files.len() {
                        let current_time = Instant::now();
                        let mouse_position = (mouse.column, mouse.row);
                        
                        // Check for double-click
                        let is_double_click = if let (Some(last_time), Some(last_pos), Some(last_idx)) = (
                            app.state.double_click_state.last_click_time,
                            app.state.double_click_state.last_click_position,
                            app.state.double_click_state.last_clicked_index,
                        ) {
                            // Check if within time interval and same position and same file
                            let elapsed = current_time.duration_since(last_time);
                            elapsed.as_millis() <= DOUBLE_CLICK_INTERVAL_MS as u128
                                && last_pos == mouse_position
                                && last_idx == clicked_index
                        } else {
                            false
                        };

                        if is_double_click {
                            // Reset double-click state
                            app.state.double_click_state.last_click_time = None;
                            app.state.double_click_state.last_click_position = None;
                            app.state.double_click_state.last_clicked_index = None;
                            
                            // Execute double-click action (navigate into directory)
                            app.state.file_list_state.select(Some(clicked_index));
                            NavigationHelper::navigate_into_directory(app)?;
                        } else {
                            // Single click - update selection and record click state
                            app.state.file_list_state.select(Some(clicked_index));
                            app.update_preview();
                            
                            // Record this click for potential double-click detection
                            app.state.double_click_state.last_click_time = Some(current_time);
                            app.state.double_click_state.last_click_position = Some(mouse_position);
                            app.state.double_click_state.last_clicked_index = Some(clicked_index);
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
        right_area: Rect,
    ) -> Result<bool> {
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                // Check if mouse is in left area (file list) or right area (preview)
                if mouse.column >= left_area.x && mouse.column < left_area.x + left_area.width {
                    // Mouse is in left panel - scroll file list
                    Ok(NavigationHelper::navigate_file_list_up(app))
                } else if mouse.column >= right_area.x
                    && mouse.column < right_area.x + right_area.width
                {
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
                } else if mouse.column >= right_area.x
                    && mouse.column < right_area.x + right_area.width
                {
                    // Mouse is in right panel - scroll preview content
                    Ok(app.scroll_preview_down())
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }

    /// Handle preview navigation keys (PageUp/PageDown)
    pub fn handle_preview_navigation(app: &mut App, key: KeyCode) -> Result<bool> {
        // Default visible height estimation (will be refined when rendering context is available)
        let default_visible_height = 20; // Reasonable default for most terminal sizes

        match key {
            KeyCode::PageUp => Ok(app.scroll_preview_page_up(default_visible_height)),
            KeyCode::PageDown => Ok(app.scroll_preview_page_down(default_visible_height)),
            _ => Ok(false),
        }
    }

    /// Reset double-click state
    pub fn reset_double_click_state(app: &mut App) {
        app.state.double_click_state.last_click_time = None;
        app.state.double_click_state.last_click_position = None;
        app.state.double_click_state.last_clicked_index = None;
    }
}
