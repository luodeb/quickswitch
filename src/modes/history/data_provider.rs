use anyhow::Result;
use std::{fs, path::PathBuf};

use crate::{
    app_state::AppState,
    config::{get_data_dir, get_history_config},
    modes::ModeAction,
    services::DataProvider,
    utils::{AppMode, DisplayItem, HistoryEntry, HistorySortMode},
};

/// Data provider for history list (History mode)
pub struct HistoryDataProvider;

impl HistoryDataProvider {
    /// Get the path to the history data file
    fn get_history_file_path(&self) -> PathBuf {
        if let Ok(data_dir) = get_data_dir() {
            data_dir.join("quickswitch.history.bin")
        } else {
            // Fallback to temp directory if data_dir cannot be created
            std::env::temp_dir().join("quickswitch.history.bin")
        }
    }

    /// Get the path to the legacy history file (for migration)
    fn get_legacy_history_file_path(&self) -> PathBuf {
        if let Ok(data_dir) = get_data_dir() {
            data_dir.join("quickswitch.history")
        } else {
            std::env::temp_dir().join("quickswitch.history")
        }
    }

    /// Load history entries from file
    fn load_history_entries(&self) -> Result<Vec<HistoryEntry>> {
        let file_path = self.get_history_file_path();

        // If the binary file exists, load from it
        if file_path.exists() {
            let data = fs::read(&file_path)?;
            match bincode::deserialize(&data) {
                Ok(entries) => return Ok(entries),
                Err(e) => {
                    // If deserialization fails, try to migrate from legacy format
                    eprintln!("Error loading history data: {}", e);
                    if let Ok(entries) = self.migrate_from_legacy() {
                        return Ok(entries);
                    }
                    return Ok(Vec::new());
                }
            }
        }

        // If binary file doesn't exist, try to migrate from legacy format
        if self.get_legacy_history_file_path().exists() {
            if let Ok(entries) = self.migrate_from_legacy() {
                return Ok(entries);
            }
        }

        // If all else fails, return empty list
        Ok(Vec::new())
    }

    /// Migrate from legacy text-based history format
    fn migrate_from_legacy(&self) -> Result<Vec<HistoryEntry>> {
        let legacy_path = self.get_legacy_history_file_path();
        if let Ok(content) = fs::read_to_string(&legacy_path) {
            let mut entries = Vec::new();

            for line in content.lines() {
                let path = PathBuf::from(line.trim());
                if path.exists() {
                    entries.push(HistoryEntry::new(path));
                }
            }

            // Save in new format
            self.save_history_entries(&entries)?;

            // Backup the legacy file
            if legacy_path.exists() {
                let backup_path = legacy_path.with_extension("history.bak");
                let _ = fs::rename(&legacy_path, backup_path);
            }

            return Ok(entries);
        }

        Ok(Vec::new())
    }

    /// Save history entries to file
    fn save_history_entries(&self, entries: &[HistoryEntry]) -> Result<()> {
        let data = bincode::serialize(entries)?;
        let file_path = self.get_history_file_path();

        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(file_path, data)?;
        Ok(())
    }

    /// Add a path to history or update its frequency if it already exists
    pub fn add_to_history(&self, path: PathBuf) -> Result<()> {
        let mut entries = self.load_history_entries()?;
        let config = get_history_config();

        // Check if path already exists in history
        let existing_index = entries.iter().position(|entry| entry.path == path);

        if let Some(index) = existing_index {
            // Update existing entry
            let mut entry = entries.remove(index);
            entry.increment_frequency();
            entries.insert(0, entry); // Move to top
        } else {
            // Add new entry
            entries.insert(0, HistoryEntry::new(path));
        }

        // Apply max entries limit
        if entries.len() > config.max_entries {
            entries.truncate(config.max_entries);
        }

        // Save updated entries
        self.save_history_entries(&entries)?;
        Ok(())
    }

    /// Get sorted history entries based on the configured sort mode
    pub fn get_sorted_entries(&self, sort_mode: &HistorySortMode) -> Result<Vec<HistoryEntry>> {
        let mut entries = self.load_history_entries()?;
        let config = get_history_config();

        // Sort entries based on the specified mode
        match sort_mode {
            HistorySortMode::Frequency => {
                entries.sort_by(|a, b| b.frequency.cmp(&a.frequency));
            }
            HistorySortMode::Recent => {
                entries.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
            }
            HistorySortMode::FrequencyRecent => {
                entries.sort_by(|a, b| {
                    let a_score = a.calculate_score(config.time_decay_days);
                    let b_score = b.calculate_score(config.time_decay_days);
                    b_score
                        .partial_cmp(&a_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            HistorySortMode::Alphabetical => {
                entries.sort_by(|a, b| {
                    let a_name = a
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    let b_name = b
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or_default();
                    a_name.cmp(b_name)
                });
            }
        }

        // Filter out entries that don't exist anymore
        entries.retain(|entry| entry.path.exists());

        Ok(entries)
    }

    /// Clean up old or low-frequency entries
    pub fn cleanup_old_entries(&self) -> Result<()> {
        let mut entries = self.load_history_entries()?;
        let config = get_history_config();

        // Remove entries with frequency below threshold
        entries.retain(|entry| entry.frequency >= config.min_frequency_threshold);

        // Save cleaned up entries
        self.save_history_entries(&entries)?;
        Ok(())
    }

    /// Get statistics about history usage
    pub fn get_statistics(&self) -> Result<HistoryStatistics> {
        let entries = self.load_history_entries()?;

        let total_entries = entries.len();
        let most_visited = entries.iter().max_by_key(|entry| entry.frequency).cloned();

        let total_frequency: u32 = entries.iter().map(|entry| entry.frequency).sum();
        let average_frequency = if !entries.is_empty() {
            total_frequency as f64 / entries.len() as f64
        } else {
            0.0
        };

        let oldest_entry = entries
            .iter()
            .min_by_key(|entry| entry.first_accessed)
            .map(|entry| entry.first_accessed);

        Ok(HistoryStatistics {
            total_entries,
            most_visited,
            average_frequency,
            oldest_entry,
        })
    }
}

/// Statistics about history usage
pub struct HistoryStatistics {
    pub total_entries: usize,
    pub most_visited: Option<HistoryEntry>,
    pub average_frequency: f64,
    pub oldest_entry: Option<chrono::DateTime<chrono::Utc>>,
}

impl DataProvider for HistoryDataProvider {
    fn navigate_into_directory(&self, state: &mut AppState) -> Result<Option<ModeAction>> {
        // In history mode, navigate to the selected directory and switch to normal mode
        if let Some(item) = state.get_selected_item() {
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
        let config = get_history_config();
        let history_entries = self.get_sorted_entries(&config.sort_mode)?;

        state.files = history_entries
            .into_iter()
            .map(DisplayItem::History)
            .collect();
        state.apply_search_filter();
        Ok(())
    }
}
