use std::path::PathBuf;

use crate::{
    app::App,
    models::{AppMode, DisplayItem},
};

/// Unified data provider trait for different modes
/// This trait provides a consistent interface for all modes to access their data
pub trait DataProvider {
    /// Get items to display for current mode
    fn get_items(&self, app: &App) -> Vec<DisplayItem>;

    /// Get current selected index
    fn get_selected_index(&self, app: &App) -> Option<usize>;

    /// Set selected index
    fn set_selected_index(&self, app: &mut App, index: Option<usize>);

    /// Get total count of items
    fn get_total_count(&self, app: &App) -> usize;

    /// Navigate up in the list
    fn navigate_up(&self, app: &mut App) -> bool;

    /// Navigate down in the list
    fn navigate_down(&self, app: &mut App) -> bool;

    /// Get the currently selected item (file or path)
    fn get_selected_item(&self, app: &App) -> Option<DisplayItem>;

    /// Get the file path for preview (unified interface)
    fn get_preview_path(&self, app: &App) -> Option<PathBuf>;

    /// Check if this provider supports directory navigation
    fn supports_directory_navigation(&self) -> bool;

    /// Update scroll offset for automatic scrolling
    fn update_scroll_offset(&self, app: &mut App, visible_height: usize);
}

/// Data provider for file list (Normal and Search modes)
pub struct FileListDataProvider;

impl DataProvider for FileListDataProvider {
    fn get_items(&self, app: &App) -> Vec<DisplayItem> {
        app.state
            .filtered_files
            .iter()
            .filter_map(|&index| app.state.files.get(index))
            .map(|file| DisplayItem::File(file.clone()))
            .collect()
    }

    fn get_selected_index(&self, app: &App) -> Option<usize> {
        app.state.file_list_state.selected()
    }

    fn set_selected_index(&self, app: &mut App, index: Option<usize>) {
        app.state.file_list_state.select(index);
    }

    fn get_total_count(&self, app: &App) -> usize {
        app.state.filtered_files.len()
    }

    fn navigate_up(&self, app: &mut App) -> bool {
        if let Some(selected) = app.state.file_list_state.selected() {
            if selected > 0 {
                app.state.file_list_state.select(Some(selected - 1));
                self.update_scroll_offset(app, 20); // Default visible height
                crate::services::PreviewManager::update_preview_from_selection(app);
                return true;
            }
        } else if !app.state.filtered_files.is_empty() {
            app.state
                .file_list_state
                .select(Some(app.state.filtered_files.len() - 1));
            self.update_scroll_offset(app, 20); // Default visible height
            crate::services::PreviewManager::update_preview_from_selection(app);
            return true;
        }
        false
    }

    fn navigate_down(&self, app: &mut App) -> bool {
        let total = app.state.filtered_files.len();
        if total == 0 {
            return false;
        }

        if let Some(selected) = app.state.file_list_state.selected() {
            if selected + 1 < total {
                app.state.file_list_state.select(Some(selected + 1));
                self.update_scroll_offset(app, 20); // Default visible height
                crate::services::PreviewManager::update_preview_from_selection(app);
                return true;
            }
        } else {
            app.state.file_list_state.select(Some(0));
            self.update_scroll_offset(app, 20); // Default visible height
            crate::services::PreviewManager::update_preview_from_selection(app);
            return true;
        }
        false
    }

    fn get_selected_item(&self, app: &App) -> Option<DisplayItem> {
        if let Some(selected) = app.state.file_list_state.selected() {
            if let Some(&file_index) = app.state.filtered_files.get(selected) {
                if let Some(file) = app.state.files.get(file_index) {
                    return Some(DisplayItem::File(file.clone()));
                }
            }
        }
        None
    }

    fn get_preview_path(&self, app: &App) -> Option<PathBuf> {
        if let Some(DisplayItem::File(file)) = self.get_selected_item(app) {
            Some(file.path)
        } else {
            None
        }
    }

    fn supports_directory_navigation(&self) -> bool {
        true
    }

    fn update_scroll_offset(&self, app: &mut App, visible_height: usize) {
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

/// Data provider for history list (History mode)
pub struct HistoryDataProvider;

impl DataProvider for HistoryDataProvider {
    fn get_items(&self, app: &App) -> Vec<DisplayItem> {
        app.state
            .filtered_history
            .iter()
            .filter_map(|&i| app.state.history.get(i))
            .map(|path| DisplayItem::HistoryPath(path.clone()))
            .collect()
    }

    fn get_selected_index(&self, app: &App) -> Option<usize> {
        app.state.history_state.selected()
    }

    fn set_selected_index(&self, app: &mut App, index: Option<usize>) {
        app.state.history_state.select(index);
    }

    fn get_total_count(&self, app: &App) -> usize {
        app.state.filtered_history.len()
    }

    fn navigate_up(&self, app: &mut App) -> bool {
        if let Some(selected) = app.state.history_state.selected() {
            if selected > 0 {
                app.state.history_state.select(Some(selected - 1));
                self.update_scroll_offset(app, 20); // Default visible height
                if let Some(path) = self.get_preview_path(app) {
                    crate::services::PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else if !app.state.filtered_history.is_empty() {
            app.state
                .history_state
                .select(Some(app.state.filtered_history.len() - 1));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                crate::services::PreviewManager::update_preview_for_path(app, &path);
            }
            return true;
        }
        false
    }

    fn navigate_down(&self, app: &mut App) -> bool {
        let total = app.state.filtered_history.len();
        if total == 0 {
            return false;
        }

        if let Some(selected) = app.state.history_state.selected() {
            if selected + 1 < total {
                app.state.history_state.select(Some(selected + 1));
                self.update_scroll_offset(app, 20); // Default visible height
                if let Some(path) = self.get_preview_path(app) {
                    crate::services::PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else {
            app.state.history_state.select(Some(0));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                crate::services::PreviewManager::update_preview_for_path(app, &path);
            }
            return true;
        }
        false
    }

    fn get_selected_item(&self, app: &App) -> Option<DisplayItem> {
        if let Some(selected) = app.state.history_state.selected() {
            if let Some(&history_index) = app.state.filtered_history.get(selected) {
                if let Some(path) = app.state.history.get(history_index) {
                    return Some(DisplayItem::HistoryPath(path.clone()));
                }
            }
        }
        None
    }

    fn get_preview_path(&self, app: &App) -> Option<PathBuf> {
        if let Some(DisplayItem::HistoryPath(path)) = self.get_selected_item(app) {
            Some(path)
        } else {
            None
        }
    }

    fn supports_directory_navigation(&self) -> bool {
        false // History mode doesn't support left/right navigation
    }

    fn update_scroll_offset(&self, app: &mut App, visible_height: usize) {
        if let Some(selected) = app.state.history_state.selected() {
            let current_offset = app.state.history_state.offset();
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
                *app.state.history_state.offset_mut() = new_offset;
            }
        }
    }
}

/// Factory function to create appropriate data provider for each mode
pub fn create_data_provider(mode: &AppMode) -> Box<dyn DataProvider> {
    match mode {
        AppMode::Normal => Box::new(FileListDataProvider),
        AppMode::History => Box::new(HistoryDataProvider),
    }
}
