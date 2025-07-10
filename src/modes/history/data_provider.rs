use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::{
    app::App,
    models::{AppMode, DisplayItem},
    modes::ModeAction,
    services::{DataProvider, PreviewManager},
};

/// Data provider for history list (History mode)
pub struct HistoryDataProvider;

impl HistoryDataProvider {

    fn get_history_file_path(&self) -> PathBuf {
        std::env::temp_dir().join("quickswitch.history")
    }

    fn load_history_from_file(&self) -> Result<Vec<PathBuf>> {
        if let Ok(content) = fs::read_to_string(&self.get_history_file_path()) {
            let history_paths: Vec<PathBuf> = content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| PathBuf::from(line.trim()))
                .filter(|path| path.exists())
                .collect();
            Ok(history_paths)
        } else {
            Ok(Vec::new())
        }
    }

    /// Save history data to file
    pub fn save_history(&self, history_paths: Vec<PathBuf>) -> Result<()> {
        let content = history_paths
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&self.get_history_file_path(), content)?;
        Ok(())
    }

    /// Add a path to history
    pub fn add_to_history(&self, path: PathBuf) -> Result<()> {
        let mut history_paths = self.load_history_from_file()?;
        history_paths.retain(|p| p != &path);
        history_paths.insert(0, path.clone());
        if history_paths.len() > 100 {
            history_paths.truncate(100);
        }

        self.save_history(history_paths)?;
        Ok(())
    }
}

impl DataProvider for HistoryDataProvider {
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
        self.get_selected_item(app)
            .map(|item| item.get_path().clone())
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
        // In history mode, navigate to the selected directory and switch to normal mode
        if let Some(item) = self.get_selected_item(app) {
            if item.is_directory() {
                // Add to history and change directory
                self.add_to_history(item.get_path().clone())?;
                app.state.current_dir = item.get_path().clone();
                return Ok(Some(ModeAction::Switch(AppMode::Normal)));
            }
        }
        Ok(Some(ModeAction::Switch(AppMode::Normal)))
    }

    fn navigate_to_parent(&self, _app: &mut App) -> Result<Option<ModeAction>> {
        // History mode doesn't support parent navigation, just switch to normal mode
        Ok(Some(ModeAction::Switch(AppMode::Normal)))
    }

    fn navigate_to_selected(&self, _app: &mut App) -> Result<bool> {
        Ok(true)
    }

    fn load_data(&self, app: &mut App) -> Result<()> {
        let history_paths = self.load_history_from_file()?;
        app.state.files = history_paths
            .into_iter()
            .map(DisplayItem::HistoryPath)
            .collect();
        app.state.apply_search_filter();
        Ok(())
    }

    fn save_position(&self, _app: &mut App) {
        // History mode doesn't need to save positions
    }

    fn restore_position(&self, app: &mut App) {
        // Initialize history mode selection
        if !app.state.filtered_files.is_empty() {
            app.state.file_list_state.select(Some(0));
        } else {
            app.state.file_list_state.select(None);
        }
    }

    fn on_directory_changed(&self, _app: &mut App, _new_dir: &PathBuf) -> Result<()> {
        // History mode doesn't handle directory changes
        Ok(())
    }
}

