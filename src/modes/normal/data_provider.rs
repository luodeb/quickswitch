use anyhow::Result;
use std::path::PathBuf;

use crate::{
    app::App,
    models::DisplayItem,
    modes::ModeAction,
    services::{DataProvider, FilesystemService, PreviewManager},
};

/// Data provider for file list (Normal and Search modes)
pub struct FileListDataProvider;

impl DataProvider for FileListDataProvider {
    fn get_items(&self, app: &App) -> Vec<DisplayItem> {
        app.state
            .filtered_files
            .iter()
            .filter_map(|&index| app.state.files.get(index))
            .cloned()
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
                if let Some(item) = self.get_selected_item(app) {
                    PreviewManager::update_preview_for_item(app, &item);
                }
                return true;
            }
        } else if !app.state.filtered_files.is_empty() {
            app.state
                .file_list_state
                .select(Some(app.state.filtered_files.len() - 1));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(item) = self.get_selected_item(app) {
                PreviewManager::update_preview_for_item(app, &item);
            }
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
                if let Some(item) = self.get_selected_item(app) {
                    PreviewManager::update_preview_for_item(app, &item);
                }
                return true;
            }
        } else {
            app.state.file_list_state.select(Some(0));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(item) = self.get_selected_item(app) {
                PreviewManager::update_preview_for_item(app, &item);
            }
            return true;
        }
        false
    }

    fn get_selected_item(&self, app: &App) -> Option<DisplayItem> {
        if let Some(selected) = app.state.file_list_state.selected() {
            if let Some(&file_index) = app.state.filtered_files.get(selected) {
                return app.state.files.get(file_index).cloned();
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

    fn update_scroll_offset(&self, app: &mut App, visible_height: usize) {
        if let Some(selected) = app.state.file_list_state.selected() {
            let current_offset = app.state.file_list_state.offset();
            let new_offset = if selected < current_offset {
                selected
            } else if selected >= current_offset + visible_height {
                selected.saturating_sub(visible_height - 1)
            } else {
                current_offset
            };

            if new_offset != current_offset {
                *app.state.file_list_state.offset_mut() = new_offset;
            }
        }
    }

    fn navigate_into_directory(&self, app: &mut App) -> Result<Option<ModeAction>> {
        if let Some(file) = self.get_selected_item(app) {
            if file.is_directory() {
                // Save current position before changing directory
                self.save_position(app);

                // Change directory
                app.state.current_dir = file.get_path().to_path_buf();

                // Handle directory change
                self.on_directory_changed(app, &app.state.current_dir.clone())?;

                return Ok(None); // Stay in current mode
            }
        }
        Ok(None)
    }

    fn navigate_to_parent(&self, app: &mut App) -> Result<Option<ModeAction>> {
        if let Some(parent) = app.state.current_dir.parent() {
            let parent_path = parent.to_path_buf();

            // Save current position before changing directory
            self.save_position(app);

            // Change directory
            app.state.current_dir = parent_path.clone();

            // Handle directory change
            self.on_directory_changed(app, &parent_path)?;

            Ok(None) // Stay in current mode
        } else {
            Ok(None)
        }
    }

    fn navigate_to_selected(&self, _app: &mut App) -> Result<bool> {
        Ok(true)
    }

    // === Data Management Methods ===

    fn load_data(&self, app: &mut App) -> Result<()> {
        let files = FilesystemService::load_directory(&app.state.current_dir)?;
        app.state.load_file_items(files);
        app.state.apply_search_filter();
        Ok(())
    }

    fn save_position(&self, app: &mut App) {
        if let Some(selected) = app.state.file_list_state.selected() {
            app.state
                .dir_positions
                .insert(app.state.current_dir.clone(), selected);
        }
    }

    fn restore_position(&self, app: &mut App) {
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
            app.state.file_list_state.select(None);
        }
    }

    fn on_directory_changed(&self, app: &mut App, _new_dir: &PathBuf) -> Result<()> {
        // Clear search and exit search mode when changing directory
        app.state.search_input.clear();
        app.state.is_searching = false;

        // Load new directory contents
        self.load_data(app)?;

        // Restore position for the new directory
        self.restore_position(app);

        // Clear preview
        PreviewManager::clear_preview(app);

        Ok(())
    }
}
