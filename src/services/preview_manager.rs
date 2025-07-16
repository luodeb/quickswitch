use crate::{
    AppState,
    services::{GlobalPreviewState, PreviewGenerator, preview::PreviewContent},
    utils::{DisplayItem, FileItem},
};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

/// Unified preview manager for handling all preview functionality
pub struct PreviewManager;

impl PreviewManager {
    pub fn preview_for_selected_item(state: &AppState) {
        if let Some(item) = state.get_selected_item() {
            // Get file info for placeholder
            let file_item = match item {
                DisplayItem::File(file) => file.clone(),
                DisplayItem::History(entry) => FileItem::from_path(&entry.path),
            };
            Self::update_preview_for_item_async(&file_item);
        }
    }

    /// Update preview for a DisplayItem with non-blocking background generation
    fn update_preview_for_item_async(file_item: &FileItem) {
        let global_state = GlobalPreviewState::instance();

        // Show immediate placeholder content
        let placeholder_title = format!("📄 {}", file_item.name);
        let placeholder_content = PreviewContent::text(vec![
            Line::from(vec![Span::styled(
                "Loading preview...".to_string(),
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(vec![Span::raw("".to_string())]),
            Line::from(vec![Span::styled(
                "Please wait while content is being processed.".to_string(),
                Style::default().fg(Color::Gray),
            )]),
        ]);
        global_state.set_current_file_item(Some(file_item.clone()));
        global_state.update_preview(
            placeholder_title,
            placeholder_content,
            Some(file_item.clone()),
        );

        // Start background task to generate actual content
        let file_path = file_item.path.clone();

        tokio::spawn(async move {
            let file_item = FileItem::from_path(&file_path);
            let (title, content) = PreviewGenerator::generate_preview_content(&file_item).await;

            // Update the global state with the actual content
            let global_state = GlobalPreviewState::instance();
            global_state.update_preview(title, content, Some(file_item));
        });
    }

    /// Clear preview content
    pub fn clear_preview() {
        let global_state = GlobalPreviewState::instance();
        global_state.clear_preview();
    }

    /// Scroll preview content up by one line
    pub fn scroll_preview_up() -> bool {
        let global_state = GlobalPreviewState::instance();
        global_state.scroll_up()
    }

    /// Scroll preview content down by one line
    pub fn scroll_preview_down() -> bool {
        let global_state = GlobalPreviewState::instance();
        global_state.scroll_down()
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_preview_page_up(visible_height: usize) -> bool {
        let global_state = GlobalPreviewState::instance();
        global_state.scroll_page_up(visible_height)
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_preview_page_down(visible_height: usize) -> bool {
        let global_state = GlobalPreviewState::instance();
        global_state.scroll_page_down(visible_height)
    }

    /// Reset preview scroll position to top
    pub fn reset_preview_scroll() {
        let global_state = GlobalPreviewState::instance();
        global_state.reset_scroll();
    }
}
