use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::{
    app_state::AppState,
    config::get_data_dir,
    modes::ModeAction,
    services::DataProvider,
    utils::{AppMode, DisplayItem},
};

/// Data provider for history list (History mode)
pub struct HistoryDataProvider;

impl HistoryDataProvider {
    fn get_history_file_path(&self) -> PathBuf {
        if let Ok(data_dir) = get_data_dir() {
            data_dir.join("quickswitch.history")
        } else {
            // Fallback to temp directory if data_dir cannot be created
            std::env::temp_dir().join("quickswitch.history")
        }
    }

    fn load_history_from_file(&self) -> Result<Vec<PathBuf>> {
        if let Ok(content) = fs::read_to_string(self.get_history_file_path()) {
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
    fn save_history_to_file(&self, history_paths: Vec<PathBuf>) -> Result<()> {
        let content = history_paths
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(self.get_history_file_path(), content)?;
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

        self.save_history_to_file(history_paths)?;
        Ok(())
    }
}

impl DataProvider for HistoryDataProvider {
    fn navigate_into_directory(&self, state: &mut AppState) -> Result<Option<ModeAction>> {
        // In history mode, navigate to the selected directory and switch to normal mode
        if let Some(item) = self.get_selected_item(state) {
            if item.is_directory() {
                // Add to history and change directory
                self.add_to_history(item.get_path().clone())?;
                state.current_dir = item.get_path().clone();
                return Ok(Some(ModeAction::Switch(AppMode::Normal)));
            }
        }
        Ok(Some(ModeAction::Switch(AppMode::Normal)))
    }

    fn load_data(&self, state: &mut AppState) -> Result<()> {
        let history_paths = self.load_history_from_file()?;
        state.files = history_paths
            .into_iter()
            .map(DisplayItem::HistoryPath)
            .collect();
        state.apply_search_filter();
        Ok(())
    }
}
