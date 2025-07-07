use std::path::PathBuf;

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
        app.update_filter();
        app.state.file_list_state.select(Some(0));
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

        if !self.state.filtered_files.is_empty() {
            self.state.file_list_state.select(Some(0));
        } else {
            self.state.file_list_state.select(None);
        }
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
                self.state.file_list_state.select(Some(1));
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
}
