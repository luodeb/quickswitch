use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::cell::RefCell;

use crate::{
    app_state::AppState,
    preview_content::{ImageState, PreviewContent},
    services::PreviewGenerator,
    utils::{DisplayItem, FileItem},
};

/// Unified preview manager for handling all preview functionality
pub struct PreviewManager;

impl PreviewManager {
    /// Update preview for a DisplayItem (unified function for files and directories)
    pub fn update_preview_for_item(state: &mut AppState, item: &DisplayItem) {
        let (title, content, image_state) = Self::generate_preview_content_for_item(state, item);
        state.preview_title = title;
        state.preview_content = content;
        state.image_state = image_state.map(RefCell::new);
        Self::reset_preview_scroll(state);
    }

    /// Clear preview content
    pub fn clear_preview(state: &mut AppState) {
        state.preview_title = "Preview".to_string();
        state.preview_content = PreviewContent::text(vec![Line::from(vec![Span::styled(
            "No file selected".to_string(),
            Style::default().fg(Color::Gray),
        )])]);
        state.image_state = None;
        Self::reset_preview_scroll(state);
    }

    /// Scroll preview content up by one line
    pub fn scroll_preview_up(state: &mut AppState) -> bool {
        if state.preview_scroll_offset > 0 {
            state.preview_scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by one line
    pub fn scroll_preview_down(state: &mut AppState) -> bool {
        if state.preview_scroll_offset + 1 < state.preview_content.len() {
            state.preview_scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_preview_page_up(state: &mut AppState, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1);
        let new_offset = state.preview_scroll_offset.saturating_sub(half_screen);
        if new_offset != state.preview_scroll_offset {
            state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_preview_page_down(state: &mut AppState, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1);
        let max_offset = state.preview_content.len().saturating_sub(visible_height);
        let new_offset = (state.preview_scroll_offset + half_screen).min(max_offset);
        if new_offset != state.preview_scroll_offset {
            state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Reset preview scroll position to top
    pub fn reset_preview_scroll(state: &mut AppState) {
        state.preview_scroll_offset = 0;
    }

    /// Generate preview content for a DisplayItem (unified function)
    fn generate_preview_content_for_item(
        state: &AppState,
        item: &DisplayItem,
    ) -> (String, PreviewContent, Option<ImageState>) {
        match item {
            DisplayItem::File(file) => PreviewGenerator::generate_preview_content(state, file),
            DisplayItem::HistoryPath(path) => {
                let file_item = FileItem::from_path(path);
                PreviewGenerator::generate_preview_content(state, &file_item)
            }
        }
    }
}
