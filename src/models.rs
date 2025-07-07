use ratatui::{text::Line, widgets::ListState};
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub struct AppState {
    pub search_input: String,
    pub current_dir: PathBuf,
    pub files: Vec<FileItem>,
    pub filtered_files: Vec<usize>,
    pub file_list_state: ListState,
    pub preview_content: Vec<Line<'static>>,
    pub preview_title: String,
    pub dir_positions: HashMap<PathBuf, usize>,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        Ok(Self {
            search_input: String::new(),
            current_dir,
            files: Vec::new(),
            filtered_files: Vec::new(),
            file_list_state: ListState::default(),
            preview_content: Vec::new(),
            preview_title: String::new(),
            dir_positions: HashMap::new(),
        })
    }
}
