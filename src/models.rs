use ratatui::{text::Line, widgets::ListState};
use std::{collections::HashMap, path::PathBuf, time::Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,  // Default navigation mode (command mode)
    Search,  // Search input mode
    History, // History selection mode
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

impl FileItem {
    pub fn from_path(path: &PathBuf) -> Self {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        let is_dir = path.is_dir();
        Self {
            name,
            path: path.to_path_buf(),
            is_dir,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DoubleClickState {
    pub last_click_time: Option<Instant>,
    pub last_click_position: Option<(u16, u16)>,
    pub last_clicked_index: Option<usize>,
}

pub struct AppState {
    pub search_input: String,
    pub current_dir: PathBuf,
    pub files: Vec<FileItem>,
    pub filtered_files: Vec<usize>,
    pub file_list_state: ListState,
    pub preview_content: Vec<Line<'static>>,
    pub preview_title: String,
    pub preview_scroll_offset: usize,
    pub dir_positions: HashMap<PathBuf, usize>,
    pub history: Vec<PathBuf>,
    pub history_state: ListState,
    pub history_file_path: PathBuf,
    pub double_click_state: DoubleClickState,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let history_file_path = std::env::temp_dir().join("quickswitch.history");
        Ok(Self {
            search_input: String::new(),
            current_dir,
            files: Vec::new(),
            filtered_files: Vec::new(),
            file_list_state: ListState::default(),
            preview_content: Vec::new(),
            preview_title: String::new(),
            preview_scroll_offset: 0,
            dir_positions: HashMap::new(),
            history: Vec::new(),
            history_state: ListState::default(),
            history_file_path,
            double_click_state: DoubleClickState {
                last_click_time: None,
                last_click_position: None,
                last_clicked_index: None,
            },
        })
    }
}
