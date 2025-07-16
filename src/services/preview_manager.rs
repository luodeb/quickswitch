use crate::{
    app_state::AppState,
    preview_content::PreviewContent,
    services::{GlobalPreviewState, PreviewGenerator},
    utils::{DisplayItem, FileItem},
};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};


/// Unified preview manager for handling all preview functionality
pub struct PreviewManager;

impl PreviewManager {
    /// Update preview for a DisplayItem with non-blocking background generation
    pub fn update_preview_for_item_async(item: &DisplayItem) {
        let global_state = GlobalPreviewState::instance();

        // Get file info for placeholder
        let file_item = match item {
            DisplayItem::File(file) => file.clone(),
            DisplayItem::History(entry) => FileItem::from_path(&entry.path),
        };

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

        global_state.update_preview(placeholder_title, placeholder_content);

        // Start background task to generate actual content
        let file_path = file_item.path.clone();

        tokio::spawn(async move {
            // Create a minimal AppState for the preview generator
            // We only need the current directory for context
            let temp_state = match AppState::new() {
                Ok(mut state) => {
                    state.current_dir = file_path.parent().unwrap_or(&file_path).to_path_buf();
                    state
                }
                Err(_) => return, // If we can't create state, abort
            };

            let file_item = FileItem::from_path(&file_path);
            let (title, content) = PreviewGenerator::generate_preview_content(&temp_state, &file_item).await;

            // Update the global state with the actual content
            let global_state = GlobalPreviewState::instance();
            global_state.update_preview(title, content);
        });
    }

    /// Legacy async method for compatibility (now just calls the non-blocking version)
    pub async fn update_preview_for_item(state: &AppState, item: &DisplayItem) {
        // For now, we'll still use the old synchronous approach for compatibility
        // but we can migrate callers to use update_preview_for_item_async
        let (title, content) = Self::generate_preview_content_for_item(state, item).await;
        let global_state = GlobalPreviewState::instance();
        global_state.update_preview(title, content);
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

    /// Generate preview content for a DisplayItem (unified function)
    async fn generate_preview_content_for_item(
        state: &AppState,
        item: &DisplayItem,
    ) -> (String, PreviewContent) {
        let file_item = match item {
            DisplayItem::File(file) => file,
            DisplayItem::History(entry) => &FileItem::from_path(&entry.path),
        };
        PreviewGenerator::generate_preview_content(state, file_item).await
    }
}
