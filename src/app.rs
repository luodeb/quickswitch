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
    pub fn new(output_file: Option<String>) -> Result<Self> {
        let mut state = AppState::new(output_file)?;
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

    pub fn reload_directory(&mut self) -> Result<()> {
        let files = filesystem::load_directory(&self.state.current_dir)?;
        self.state.files = files;
        Ok(())
    }
}
