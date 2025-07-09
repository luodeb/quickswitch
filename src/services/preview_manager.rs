use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::path::Path;

use crate::{app::App, models::FileItem, services::FilesystemService};

/// Unified preview manager for handling all preview functionality
pub struct PreviewManager;

impl PreviewManager {
    /// Update preview based on current selection
    pub fn update_preview_from_selection(app: &mut App) {
        if let Some(file) = app.get_selected_file().cloned() {
            Self::update_preview_for_file(app, &file);
        } else {
            Self::clear_preview(app);
        }
    }

    /// Update preview for a specific file
    pub fn update_preview_for_file(app: &mut App, file: &FileItem) {
        let (title, content) = FilesystemService::generate_preview_content(file);
        app.state.preview_title = title;
        app.state.preview_content = content;
        Self::reset_preview_scroll(app);
    }

    /// Update preview for a specific path
    pub fn update_preview_for_path(app: &mut App, path: &Path) {
        let file_item = FileItem::from_path(path);
        Self::update_preview_for_file(app, &file_item);
    }

    /// Clear preview content
    pub fn clear_preview(app: &mut App) {
        app.state.preview_title = "Preview".to_string();
        app.state.preview_content = vec![Line::from(vec![Span::styled(
            "No file selected".to_string(),
            Style::default().fg(Color::Gray),
        )])];
        Self::reset_preview_scroll(app);
    }

    /// Scroll preview content up by one line
    pub fn scroll_preview_up(app: &mut App) -> bool {
        if app.state.preview_scroll_offset > 0 {
            app.state.preview_scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by one line
    pub fn scroll_preview_down(app: &mut App) -> bool {
        if app.state.preview_scroll_offset + 1 < app.state.preview_content.len() {
            app.state.preview_scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_preview_page_up(app: &mut App, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1);
        let new_offset = app.state.preview_scroll_offset.saturating_sub(half_screen);
        if new_offset != app.state.preview_scroll_offset {
            app.state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_preview_page_down(app: &mut App, visible_height: usize) -> bool {
        let half_screen = (visible_height / 2).max(1);
        let max_offset = app
            .state
            .preview_content
            .len()
            .saturating_sub(visible_height);
        let new_offset = (app.state.preview_scroll_offset + half_screen).min(max_offset);
        if new_offset != app.state.preview_scroll_offset {
            app.state.preview_scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Reset preview scroll position to top
    pub fn reset_preview_scroll(app: &mut App) {
        app.state.preview_scroll_offset = 0;
    }
}
