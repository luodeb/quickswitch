use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, StatefulWidget},
};
use ratatui_image::StatefulImage;

use super::Renderer;
use crate::{AppState, preview_content::PreviewContent};

/// Renderer for preview panel showing file/directory content
#[derive(Default)]
pub struct PreviewRenderer;

impl PreviewRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for PreviewRenderer {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        match &state.preview_content {
            PreviewContent::Text(lines) => {
                self.render_text_preview(f, area, state, lines);
            }
            PreviewContent::Image(protocol) => {
                self.render_image_preview(f, area, state, protocol);
            }
        }
    }
}

impl PreviewRenderer {
    /// Render text preview content
    fn render_text_preview(&self, f: &mut Frame, area: Rect, state: &AppState, lines: &[ratatui::text::Line<'static>]) {
        // Calculate the visible content based on scroll offset
        let total_lines = lines.len();
        let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
        let scroll_offset = state.preview_scroll_offset;

        // Determine the range of lines to display
        let start_line = scroll_offset;
        let end_line = (start_line + visible_height).min(total_lines);

        // Get the visible content slice
        let visible_content: Vec<_> = if start_line < total_lines {
            lines[start_line..end_line]
                .iter()
                .map(|line| ListItem::new(line.clone()))
                .collect()
        } else {
            vec![]
        };

        let preview_list = List::new(visible_content).block(
            Block::default()
                .borders(Borders::ALL)
                .title(&*state.preview_title),
        );

        f.render_widget(preview_list, area);
    }

    /// Render image preview content
    fn render_image_preview(&self, f: &mut Frame, area: Rect, state: &AppState, _protocol: &Box<dyn ratatui_image::protocol::StatefulProtocol>) {
        // Check if we have image state available
        if let Some(image_state_cell) = &state.image_state {
            // Try to borrow the image state mutably
            if let Ok(mut image_state) = image_state_cell.try_borrow_mut() {
                // Create the StatefulImage widget
                let image_widget = StatefulImage::new(None);

                // Render the image with the protocol state
                image_widget.render(area, f.buffer_mut(), &mut image_state.protocol);
                return;
            }
        }

        // Fallback: render a message indicating image preview is available
        let preview_list = List::new(vec![
            ListItem::new("🖼️ Image Preview"),
            ListItem::new(""),
            ListItem::new("Image loaded successfully!"),
            ListItem::new("Rendering with StatefulImage widget..."),
        ]).block(
            Block::default()
                .borders(Borders::ALL)
                .title(&*state.preview_title),
        );

        f.render_widget(preview_list, area);
    }
}
