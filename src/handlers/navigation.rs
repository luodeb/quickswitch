use anyhow::Result;
use ratatui::widgets::ListState;

use crate::app::App;

/// Common navigation operations for list-based interfaces
pub struct NavigationHelper;

impl NavigationHelper {
    /// Navigate up in a list
    pub fn navigate_up(state: &mut ListState, _list_len: usize) -> bool {
        if let Some(selected) = state.selected() {
            if selected > 0 {
                state.select(Some(selected - 1));
                return true;
            }
        }
        false
    }

    /// Navigate down in a list
    pub fn navigate_down(state: &mut ListState, list_len: usize) -> bool {
        if let Some(selected) = state.selected() {
            if selected < list_len.saturating_sub(1) {
                state.select(Some(selected + 1));
                return true;
            }
        } else if list_len > 0 {
            state.select(Some(0));
            return true;
        }
        false
    }

    /// Navigate to parent directory
    pub fn navigate_to_parent(app: &mut App) -> Result<bool> {
        if let Some(parent) = app.state.current_dir.parent() {
            app.change_directory(parent.to_path_buf())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Navigate into selected directory
    pub fn navigate_into_directory(app: &mut App) -> Result<bool> {
        if let Some(file) = app.get_selected_file() {
            if file.is_dir {
                app.change_directory(file.path.clone())?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Navigate up in file list and update preview
    pub fn navigate_file_list_up(app: &mut App) -> bool {
        let changed = Self::navigate_up(
            &mut app.state.file_list_state,
            app.state.filtered_files.len(),
        );
        if changed {
            // Update offset for automatic scrolling (assuming 20 as default visible height)
            Self::update_list_offset(app, 20);
            app.update_preview();
        }
        changed
    }

    /// Navigate down in file list and update preview
    pub fn navigate_file_list_down(app: &mut App) -> bool {
        let changed = Self::navigate_down(
            &mut app.state.file_list_state,
            app.state.filtered_files.len(),
        );
        if changed {
            // Update offset for automatic scrolling (assuming 20 as default visible height)
            Self::update_list_offset(app, 20);
            app.update_preview();
        }
        changed
    }

    /// Navigate up in history list
    pub fn navigate_history_up(app: &mut App) -> bool {
        Self::navigate_up(&mut app.state.history_state, app.state.history.len())
    }

    /// Navigate down in history list
    pub fn navigate_history_down(app: &mut App) -> bool {
        Self::navigate_down(&mut app.state.history_state, app.state.history.len())
    }

    /// Update list offset for automatic scrolling when selection moves beyond half of visible area
    fn update_list_offset(app: &mut App, visible_height: usize) {
        if let Some(selected) = app.state.file_list_state.selected() {
            let current_offset = app.state.file_list_state.offset();
            let half_visible = visible_height / 2;

            let new_offset = if selected >= current_offset + visible_height {
                // 向下滚动：选中项超出底部
                selected.saturating_sub(half_visible)
            } else if selected < current_offset {
                // 向上滚动：选中项超出顶部
                selected.saturating_sub(half_visible)
            } else {
                // 选中项在可见范围内，检查是否需要居中滚动
                if selected > current_offset + half_visible + 2 {
                    // 选中项在下半部分，向下滚动
                    selected.saturating_sub(half_visible)
                } else if selected < current_offset + half_visible.saturating_sub(2) {
                    // 选中项在上半部分，向上滚动
                    selected.saturating_sub(half_visible)
                } else {
                    current_offset
                }
            };

            if new_offset != current_offset {
                *app.state.file_list_state.offset_mut() = new_offset;
            }
        }
    }
}
