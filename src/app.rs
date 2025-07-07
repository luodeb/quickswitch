use std::path::PathBuf;
use std::fs;

use anyhow::Result;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    filesystem,
    models::{AppState, FileItem},
};

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut state = AppState::new()?;
        let files = filesystem::load_directory(&state.current_dir)?;
        state.files = files;

        let mut app = Self { state };
        app.load_history().unwrap_or(()); // Ignore errors when loading history
        app.update_filter();
        app.state.file_list_state.select(None);
        app.update_preview();
        Ok(app)
    }

    pub fn update_filter(&mut self) {
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

        self.state.file_list_state.select(None);
        
    }

    pub fn get_selected_file(&self) -> Option<&FileItem> {
        if let Some(selected) = self.state.file_list_state.selected() {
            if let Some(&file_index) = self.state.filtered_files.get(selected) {
                return self.state.files.get(file_index);
            }
        }
        None
    }

    pub fn update_preview(&mut self) {
        if let Some(file) = self.get_selected_file() {
            let (title, content) = filesystem::generate_preview_content(file);
            self.state.preview_title = title;
            self.state.preview_content = content;
        } else {
            self.state.preview_title = "Preview".to_string();
            self.state.preview_content = vec![Line::from(vec![Span::styled(
                "No file selected".to_string(),
                Style::default().fg(Color::Gray),
            )])];
        }
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
            // 如果没有保存的位置，默认选择第一个
            if !self.state.filtered_files.is_empty() {
                self.state.file_list_state.select(None);
            } else {
                self.state.file_list_state.select(None);
            }
        }
    }

    pub fn change_directory(&mut self, new_dir: PathBuf) -> Result<()> {
        self.save_current_position();

        self.state.current_dir = new_dir;
        self.reload_directory()?;
        self.state.search_input.clear();
        self.update_filter();

        self.restore_position();
        self.update_preview();

        Ok(())
    }

    pub fn reload_directory(&mut self) -> Result<()> {
        let files = filesystem::load_directory(&self.state.current_dir)?;
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
        }
        Ok(())
    }

    pub fn save_history(&self) -> Result<()> {
        let content = self.state.history
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

    pub fn enter_search_mode(&mut self) {
        self.state.mode = crate::models::AppMode::Search;
    }

    pub fn enter_history_mode(&mut self) {
        self.state.mode = crate::models::AppMode::History;
        if !self.state.history.is_empty() {
            self.state.history_state.select(Some(0));
        }
    }

    pub fn enter_normal_mode(&mut self) {
        self.state.mode = crate::models::AppMode::Normal;
    }
}
