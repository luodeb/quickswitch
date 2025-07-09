use anyhow::Result;
use std::path::PathBuf;

use crate::{
    app::App,
    models::DisplayItem,
    modes::ModeAction,
    services::{DataProvider, PreviewManager},
};

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
                if let Some(path) = self.get_preview_path(app) {
                    PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else if !app.state.filtered_files.is_empty() {
            app.state
                .file_list_state
                .select(Some(app.state.filtered_files.len() - 1));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                PreviewManager::update_preview_for_path(app, &path);
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
                if let Some(path) = self.get_preview_path(app) {
                    PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else {
            app.state.file_list_state.select(Some(0));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                PreviewManager::update_preview_for_path(app, &path);
            }
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
        if let Some(file) = app.get_selected_file() {
            if file.is_dir {
                app.change_directory(file.path.clone())?;
                return Ok(None); // Stay in current mode
            }
        }
        Ok(None)
    }

    fn navigate_to_parent(&self, app: &mut App) -> Result<Option<ModeAction>> {
        if let Some(parent) = app.state.current_dir.parent() {
            app.change_directory(parent.to_path_buf())?;
            Ok(None) // Stay in current mode
        } else {
            Ok(None)
        }
    }
}
