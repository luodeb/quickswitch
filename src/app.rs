use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::{
    models::{AppState, FileItem},
    services::{FilesystemService, PreviewManager},
};

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut state = AppState::new()?;
        let files = FilesystemService::load_directory(&state.current_dir)?;
        state.files = files;

        let mut app = Self { state };
        app.load_history().unwrap_or(()); // Ignore errors when loading history
        app.update_filter();
        app.state.file_list_state.select(None);
        PreviewManager::update_preview_from_selection(&mut app);
        Ok(app)
    }

    pub fn update_filter(&mut self) {
        // Filter files
        if self.state.search_input.is_empty() {
            self.state.filtered_files = (0..self.state.files.len()).collect();
        } else {
            let search_lower = self.state.search_input.to_lowercase();
            self.state.filtered_files = self
                .state
                .files
                .iter()
                .enumerate()
                .filter(|(_, file)| file.name.to_lowercase().contains(&search_lower))
                .map(|(i, _)| i)
                .collect();
        }

        // Filter history
        if self.state.search_input.is_empty() {
            self.state.filtered_history = (0..self.state.history.len()).collect();
        } else {
            let search_lower = self.state.search_input.to_lowercase();
            self.state.filtered_history = self
                .state
                .history
                .iter()
                .enumerate()
                .filter(|(_, path)| {
                    path.to_string_lossy()
                        .to_lowercase()
                        .contains(&search_lower)
                })
                .map(|(i, _)| i)
                .collect();
        }

        self.state.file_list_state.select(None);
        self.state.history_state.select(None);
    }

    pub fn get_selected_file(&self) -> Option<&FileItem> {
        if let Some(selected) = self.state.file_list_state.selected() {
            if let Some(&file_index) = self.state.filtered_files.get(selected) {
                return self.state.files.get(file_index);
            }
        }
        None
    }

    fn save_current_position(&mut self) {
        if let Some(selected) = self.state.file_list_state.selected() {
            self.state
                .dir_positions
                .insert(self.state.current_dir.clone(), selected);
        }
    }

    fn restore_position(&mut self) {
        if let Some(&saved_position) = self.state.dir_positions.get(&self.state.current_dir) {
            // 确保保存的位置在当前过滤结果范围内
            if saved_position < self.state.filtered_files.len() {
                self.state.file_list_state.select(Some(saved_position));
            } else {
                // 如果保存的位置超出范围，选择最后一个
                if !self.state.filtered_files.is_empty() {
                    self.state
                        .file_list_state
                        .select(Some(self.state.filtered_files.len() - 1));
                } else {
                    self.state.file_list_state.select(None);
                }
            }
        } else {
            self.state.file_list_state.select(None);
        }
    }

    pub fn change_directory(&mut self, new_dir: PathBuf) -> Result<()> {
        self.save_current_position();

        self.state.current_dir = new_dir;
        self.reload_directory()?;

        // Clear search and exit search mode when changing directory
        self.state.search_input.clear();
        self.state.is_searching = false;
        self.update_filter();

        self.restore_position();
        PreviewManager::update_preview_from_selection(self);

        Ok(())
    }

    pub fn reload_directory(&mut self) -> Result<()> {
        let files = FilesystemService::load_directory(&self.state.current_dir)?;
        self.state.files = files;
        Ok(())
    }

    pub fn load_history(&mut self) -> Result<()> {
        if let Ok(content) = fs::read_to_string(&self.state.history_file_path) {
            self.state.history = content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| PathBuf::from(line.trim()))
                .filter(|path| path.exists())
                .collect();

            // Initialize filtered_history with all indices
            self.state.filtered_history = (0..self.state.history.len()).collect();
        }
        Ok(())
    }

    pub fn save_history(&self) -> Result<()> {
        let content = self
            .state
            .history
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&self.state.history_file_path, content)?;
        Ok(())
    }

    pub fn add_to_history(&mut self, path: PathBuf) -> Result<()> {
        // Remove existing entry if present
        self.state.history.retain(|p| p != &path);

        // Add to front
        self.state.history.insert(0, path);

        // Limit history size to 100 entries
        if self.state.history.len() > 100 {
            self.state.history.truncate(100);
        }

        self.save_history()?;
        Ok(())
    }

    /// Scroll preview content up by one line
    pub fn scroll_preview_up(&mut self) -> bool {
        if self.state.preview_scroll_offset > 0 {
            self.state.preview_scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by one line
    pub fn scroll_preview_down(&mut self) -> bool {
        if self.state.preview_scroll_offset + 1 < self.state.preview_content.len() {
            self.state.preview_scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Reset preview scroll position to top
    pub fn reset_preview_scroll(&mut self) {
        self.state.preview_scroll_offset = 0;
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_preview_page_up(&mut self, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1); // At least scroll 1 line
        let new_offset = self.state.preview_scroll_offset.saturating_sub(half_screen);
        if new_offset != self.state.preview_scroll_offset {
            self.state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_preview_page_down(&mut self, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1); // At least scroll 1 line
        let max_offset = self
            .state
            .preview_content
            .len()
            .saturating_sub(visible_height);
        let new_offset = (self.state.preview_scroll_offset + half_screen).min(max_offset);
        if new_offset != self.state.preview_scroll_offset {
            self.state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }
}
