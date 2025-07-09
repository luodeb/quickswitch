use anyhow::Result;
use std::path::PathBuf;

use crate::{
    app::App,
    models::{AppMode, DisplayItem},
    modes::ModeAction,
    services::{DataProvider, PreviewManager},
};

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
                    PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else if !app.state.filtered_history.is_empty() {
            app.state
                .history_state
                .select(Some(app.state.filtered_history.len() - 1));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                PreviewManager::update_preview_for_path(app, &path);
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
                    PreviewManager::update_preview_for_path(app, &path);
                }
                return true;
            }
        } else {
            app.state.history_state.select(Some(0));
            self.update_scroll_offset(app, 20); // Default visible height
            if let Some(path) = self.get_preview_path(app) {
                PreviewManager::update_preview_for_path(app, &path);
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

    fn update_scroll_offset(&self, app: &mut App, visible_height: usize) {
        if let Some(selected) = app.state.history_state.selected() {
            let current_offset = app.state.history_state.offset();
            let new_offset = if selected < current_offset {
                selected
            } else if selected >= current_offset + visible_height {
                selected.saturating_sub(visible_height - 1)
            } else {
                current_offset
            };

            if new_offset != current_offset {
                *app.state.history_state.offset_mut() = new_offset;
            }
        }
    }

    fn navigate_into_directory(&self, app: &mut App) -> Result<Option<ModeAction>> {
        // In history mode, navigate to the selected directory and switch to normal mode
        if let Some(DisplayItem::HistoryPath(path)) = self.get_selected_item(app) {
            if path.is_dir() {
                app.change_directory(path)?;
                return Ok(Some(ModeAction::Switch(AppMode::Normal)));
            }
        }
        Ok(Some(ModeAction::Switch(AppMode::Normal)))
    }

    fn navigate_to_parent(&self, _app: &mut App) -> Result<Option<ModeAction>> {
        // History mode doesn't support parent navigation, just switch to normal mode
        Ok(Some(ModeAction::Switch(AppMode::Normal)))
    }
}
