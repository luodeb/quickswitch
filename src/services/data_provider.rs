use anyhow::Result;
use std::path::PathBuf;

use crate::{
    app::App,
    models::{AppMode, DisplayItem},
    modes::{ModeAction, history, normal},
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

    /// Update scroll offset for automatic scrolling
    fn update_scroll_offset(&self, app: &mut App, visible_height: usize);

    /// Navigate into the selected directory (if applicable)
    /// Returns Some(ModeAction) if mode should change, None if should stay in current mode
    fn navigate_into_directory(&self, app: &mut App) -> Result<Option<ModeAction>>;

    /// Navigate to parent directory (if applicable)
    /// Returns Some(ModeAction) if mode should change, None if should stay in current mode
    fn navigate_to_parent(&self, app: &mut App) -> Result<Option<ModeAction>>;
}

/// Factory function to create appropriate data provider for each mode
pub fn create_data_provider(mode: &AppMode) -> Box<dyn DataProvider> {
    match mode {
        AppMode::Normal => Box::new(normal::FileListDataProvider),
        AppMode::History => Box::new(history::HistoryDataProvider),
    }
}
