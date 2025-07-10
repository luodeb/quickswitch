use std::path::PathBuf;

use crate::{app::App, models::DisplayItem};

/// Service for managing application state transitions and operations
pub struct StateService;

impl StateService {
    /// Apply search filter to file list
    pub fn apply_search_filter(app: &mut App) {
        app.state.apply_search_filter();
    }

    /// Save current position for the current directory
    pub fn save_current_position(app: &mut App) {
        if let Some(selected) = app.state.file_list_state.selected() {
            app.state
                .dir_positions
                .insert(app.state.current_dir.clone(), selected);
        }
    }

    /// Restore position for the current directory
    pub fn restore_position(app: &mut App) {
        if let Some(&saved_position) = app.state.dir_positions.get(&app.state.current_dir) {
            // 确保保存的位置在当前过滤结果范围内
            if saved_position < app.state.filtered_files.len() {
                app.state.file_list_state.select(Some(saved_position));
            } else {
                // 如果保存的位置超出范围，选择最后一个
                if !app.state.filtered_files.is_empty() {
                    app.state
                        .file_list_state
                        .select(Some(app.state.filtered_files.len() - 1));
                } else {
                    app.state.file_list_state.select(None);
                }
            }
        } else {
            // 如果没有保存的位置，默认不选择任何项
            app.state.file_list_state.select(None);
        }
    }

    /// Initialize history state when entering history mode
    pub fn initialize_history_mode(app: &mut App) {
        if !app.state.filtered_files.is_empty() {
            app.state.file_list_state.select(Some(0));
        } else {
            app.state.file_list_state.select(None);
        }
    }

    /// Move history item to front (used when selecting from history)
    pub fn move_history_to_front(app: &mut App, index: usize) -> Option<PathBuf> {
        if let Some(&file_index) = app.state.filtered_files.get(index) {
            if let Some(DisplayItem::HistoryPath(path)) = app.state.files.get(file_index).cloned() {
                // Remove the item from its current position
                app.state.files.remove(file_index);
                // Insert at the front
                app.state
                    .files
                    .insert(0, DisplayItem::HistoryPath(path.clone()));
                // Update filtered_files indices
                app.state.apply_search_filter();
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    }
}
