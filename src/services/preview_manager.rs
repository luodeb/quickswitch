use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{app::App, models::DisplayItem, FileItem, FilesystemService};

/// Unified preview manager for handling all preview functionality
pub struct PreviewManager;

impl PreviewManager {
    /// Update preview for a DisplayItem (unified function for files and directories)
    pub fn update_preview_for_item(app: &mut App, item: &DisplayItem) {
        let (title, content) = Self::generate_preview_content_for_item(item);
        app.state.preview_title = title;
        app.state.preview_content = content;
        Self::reset_preview_scroll(app);
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

    /// Generate preview content for a DisplayItem (unified function)
    fn generate_preview_content_for_item(item: &DisplayItem) -> (String, Vec<Line<'static>>) {
        match item {
            DisplayItem::File(file) => FilesystemService::generate_preview_content(file),
            DisplayItem::HistoryPath(path) => {
                let file_item = FileItem::from_path(path);
                FilesystemService::generate_preview_content(&file_item)
            }
        }
    }
}
