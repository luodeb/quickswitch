use ratatui::widgets::ListState;
use std::{collections::HashMap, path::PathBuf, time::Instant};

use crate::{
    core::layout::LayoutManager,
    utils::{DisplayItem, FileItem},
};

#[derive(Clone, Debug)]
pub struct DoubleClickState {
    pub last_click_time: Option<Instant>,
    pub last_click_position: Option<(u16, u16)>,
    pub last_clicked_index: Option<usize>,
}

pub struct AppState {
    pub search_input: String,
    pub is_searching: bool,
    pub show_hidden_files: bool,
    pub current_dir: PathBuf,
    pub files: Vec<DisplayItem>,
    pub filtered_files: Vec<usize>,
    pub file_list_state: ListState,
    pub dir_positions: HashMap<PathBuf, usize>,
    pub double_click_state: DoubleClickState,
    pub layout: LayoutManager,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        Ok(Self {
            search_input: String::new(),
            is_searching: false,
            show_hidden_files: false,
            current_dir,
            files: Vec::new(),
            filtered_files: Vec::new(),
            file_list_state: ListState::default(),
            dir_positions: HashMap::new(),
            double_click_state: DoubleClickState {
                last_click_time: None,
                last_click_position: None,
                last_clicked_index: None,
            },
            layout: LayoutManager::new(),
        })
    }

    /// Update the layout based on terminal size
    pub fn update_layout(&mut self, terminal_size: ratatui::layout::Rect) {
        self.layout.update_layout(terminal_size);
    }

    /// Check if a point is in the left panel area
    pub fn is_point_in_left_panel(&self, x: u16, y: u16) -> bool {
        self.layout.is_in_left_area(x, y)
    }

    /// Check if a point is in the right panel area
    pub fn is_point_in_right_panel(&self, x: u16, y: u16) -> bool {
        self.layout.is_in_right_area(x, y)
    }

    /// Check if a point is in the search area
    pub fn is_point_in_search_area(&self, x: u16, y: u16) -> bool {
        self.layout.is_in_search_area(x, y)
    }

    /// Load file items for Normal mode
    pub fn load_file_items(&mut self, file_items: Vec<FileItem>) {
        self.files = file_items.into_iter().map(DisplayItem::File).collect();
        self.reset_filter();
    }

    /// Reset filter and selection
    pub fn reset_filter(&mut self) {
        self.filtered_files = self
            .files
            .iter()
            .enumerate()
            .filter(|(_, item)| self.should_show_item(item))
            .map(|(i, _)| i)
            .collect();
        self.file_list_state.select(None);
    }

    /// Apply search filter to current items
    pub fn apply_search_filter(&mut self) {
        if self.search_input.is_empty() {
            self.filtered_files = self
                .files
                .iter()
                .enumerate()
                .filter(|(_, item)| self.should_show_item(item))
                .map(|(i, _)| i)
                .collect();
        } else {
            let search_lower = self.search_input.to_lowercase();
            self.filtered_files = self
                .files
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    self.should_show_item(item)
                        && item
                            .get_display_name()
                            .to_lowercase()
                            .contains(&search_lower)
                })
                .map(|(i, _)| i)
                .collect();
        }
        self.file_list_state.select(None);
    }

    /// Get selected item
    pub fn get_selected_item(&self) -> Option<DisplayItem> {
        if let Some(selected) = self.file_list_state.selected() {
            if let Some(&file_index) = self.filtered_files.get(selected) {
                return self.files.get(file_index).cloned();
            }
        }
        None
    }

    /// Check if an item should be shown based on current filter settings
    fn should_show_item(&self, item: &DisplayItem) -> bool {
        // Always show non-file items (like history entries)
        if !matches!(item, DisplayItem::File(_)) {
            return true;
        }

        let name = item.get_display_name();

        // Check if it's a hidden file (starts with '.')
        if name.starts_with('.') {
            // Show hidden files only if show_hidden_files is true
            self.show_hidden_files
        } else {
            // Always show non-hidden files
            true
        }
    }

    /// Toggle hidden files visibility and reapply filters
    pub fn toggle_hidden_files(&mut self) {
        self.show_hidden_files = !self.show_hidden_files;
        self.apply_search_filter();
    }
}
