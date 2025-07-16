use crate::utils::FileItem;

use super::preview::PreviewContent;
use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::sync::{Arc, RwLock};

/// Global preview state that can be safely accessed from multiple threads
#[derive(Debug, Clone)]
pub struct PreviewState {
    pub content: PreviewContent,
    pub title: String,
    pub scroll_offset: usize,
    pub current_file_item: Option<FileItem>,
}

impl Default for PreviewState {
    fn default() -> Self {
        Self {
            content: PreviewContent::text(vec![Line::from(vec![Span::styled(
                "No file selected".to_string(),
                Style::default().fg(Color::Gray),
            )])]),
            title: "Preview".to_string(),
            scroll_offset: 0,
            current_file_item: None,
        }
    }
}

/// Global preview state manager with thread-safe access
pub struct GlobalPreviewState {
    state: Arc<RwLock<PreviewState>>,
}

impl GlobalPreviewState {
    /// Create a new global preview state
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(PreviewState::default())),
        }
    }

    pub fn set_current_file_item(&self, path: Option<FileItem>) {
        let mut state = self.state.write().unwrap();
        state.current_file_item = path;
    }

    fn get_current_file_item(&self) -> Option<FileItem> {
        self.state.read().unwrap().current_file_item.clone()
    }

    /// Get a copy of the current preview state
    pub fn get_state(&self) -> PreviewState {
        self.state.read().unwrap().clone()
    }

    /// Update the preview content and title
    pub fn update_preview(
        &self,
        title: String,
        content: PreviewContent,
        file_item: Option<FileItem>,
    ) {
        if file_item != self.get_current_file_item() {
            return;
        }
        let mut state = self.state.write().unwrap();
        state.title = title;
        state.content = content;
        state.scroll_offset = 0; // Reset scroll when content changes
    }

    /// Clear the preview content
    pub fn clear_preview(&self) {
        let mut state = self.state.write().unwrap();
        state.title = "Preview".to_string();
        state.content = PreviewContent::text(vec![Line::from(vec![Span::styled(
            "No file selected".to_string(),
            Style::default().fg(Color::Gray),
        )])]);
        state.scroll_offset = 0;
    }

    /// Get the current preview title
    pub fn get_title(&self) -> String {
        self.state.read().unwrap().title.clone()
    }

    /// Get the current preview content
    pub fn get_content(&self) -> PreviewContent {
        self.state.read().unwrap().content.clone()
    }

    /// Get the current scroll offset
    pub fn get_scroll_offset(&self) -> usize {
        self.state.read().unwrap().scroll_offset
    }

    /// Set the scroll offset
    pub fn set_scroll_offset(&self, offset: usize) {
        let mut state = self.state.write().unwrap();
        state.scroll_offset = offset;
    }

    /// Scroll preview content up by one line
    pub fn scroll_up(&self) -> bool {
        let mut state = self.state.write().unwrap();
        if state.scroll_offset > 0 {
            state.scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by one line
    pub fn scroll_down(&self) -> bool {
        let mut state = self.state.write().unwrap();
        if state.scroll_offset + 1 < state.content.len() {
            state.scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_page_up(&self, visible_height: usize) -> bool {
        let mut state = self.state.write().unwrap();
        let half_screen = (visible_height / 2).max(1);
        let new_offset = state.scroll_offset.saturating_sub(half_screen);
        if new_offset != state.scroll_offset {
            state.scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_page_down(&self, visible_height: usize) -> bool {
        let mut state = self.state.write().unwrap();
        let half_screen = (visible_height / 2).max(1);
        let max_offset = state.content.len().saturating_sub(visible_height);
        let new_offset = (state.scroll_offset + half_screen).min(max_offset);
        if new_offset != state.scroll_offset {
            state.scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Reset scroll position to top
    pub fn reset_scroll(&self) {
        let mut state = self.state.write().unwrap();
        state.scroll_offset = 0;
    }
}

impl Default for GlobalPreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global instance of the preview state
pub static GLOBAL_PREVIEW_STATE: Lazy<GlobalPreviewState> = Lazy::new(GlobalPreviewState::new);

/// Convenience functions for accessing the global preview state
impl GlobalPreviewState {
    /// Get the global preview state instance
    pub fn instance() -> &'static GlobalPreviewState {
        &GLOBAL_PREVIEW_STATE
    }
}
