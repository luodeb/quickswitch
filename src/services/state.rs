use std::path::PathBuf;

use crate::app::App;

/// Service for managing application state transitions and operations
pub struct StateService;

impl StateService {
    /// Apply search filter to file list
    pub fn apply_search_filter(app: &mut App) {
        if app.state.search_input.is_empty() {
            app.state.filtered_files = (0..app.state.files.len()).collect();
        } else {
            let search_lower = app.state.search_input.to_lowercase();
            app.state.filtered_files = app
                .state
                .files
                .iter()
                .enumerate()
                .filter(|(_, file)| file.name.to_lowercase().contains(&search_lower))
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection when filter changes
        app.state.file_list_state.select(None);
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
        if !app.state.history.is_empty() {
            app.state.history_state.select(Some(0));
        } else {
            app.state.history_state.select(None);
        }
    }

    /// Move history item to front (used when selecting from history)
    pub fn move_history_to_front(app: &mut App, index: usize) -> Option<PathBuf> {
        if index < app.state.history.len() {
            let selected_path = app.state.history.remove(index);
            app.state.history.insert(0, selected_path.clone());
            Some(selected_path)
        } else {
            None
        }
    }
}
