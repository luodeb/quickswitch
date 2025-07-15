use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};
use ratatui_image::{StatefulImage, protocol::StatefulProtocol};

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
                if let Ok(mut protocol) = protocol.try_borrow_mut() {
                    self.render_image_preview(f, area, state, &mut protocol);
                }
            }
        }
    }
}

impl PreviewRenderer {
    /// Render text preview content
    fn render_text_preview(
        &self,
        f: &mut Frame,
        area: Rect,
        state: &AppState,
        lines: &[ratatui::text::Line<'static>],
    ) {
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
    fn render_image_preview(
        &self,
        f: &mut Frame,
        area: Rect,
        state: &AppState,
        protocol: &mut StatefulProtocol,
    ) {
        // Create the StatefulImage widget with a border
        let block = Block::default()
            .borders(Borders::ALL)
            .title(&*state.preview_title);
        let inner_area = block.inner(area);

        // Render the block first
        f.render_widget(block, area);

        // Create and render the StatefulImage widget
        let image_widget = StatefulImage::default();
        f.render_stateful_widget(image_widget, inner_area, protocol);

        // Handle encoding result (important for ratatui-image 8.0)
        if let Some(Err(_e)) = protocol.last_encoding_result() {
            // If there's an encoding error, we could log it or show an error message
            // For now, we'll just continue - the image might still render partially
        }
    }
}
