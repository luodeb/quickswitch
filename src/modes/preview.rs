use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};
use ratatui_image::{picker::Picker, StatefulImage};

use super::Renderer;
use crate::{AppState, app_state::PreviewType};

/// Renderer for preview panel showing file/directory content
#[derive(Default)]
pub struct PreviewRenderer {
    image_picker: Option<Picker>,
}

impl PreviewRenderer {
    pub fn new() -> Self {
        Self {
            image_picker: None,
        }
    }

    fn get_or_create_picker(&mut self) -> &mut Picker {
        if self.image_picker.is_none() {
            self.image_picker = Some(Picker::new((8, 12)));
        }
        self.image_picker.as_mut().unwrap()
    }
}

impl Renderer for PreviewRenderer {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        match &state.preview_type {
            PreviewType::Text => {
                self.render_text_preview(f, area, state);
            }
            PreviewType::Image { path } => {
                self.render_image_preview(f, area, state, path);
            }
        }
    }
}

impl PreviewRenderer {
    fn render_text_preview(&self, f: &mut Frame, area: Rect, state: &AppState) {
        // Calculate the visible content based on scroll offset
        let total_lines = state.preview_content.len();
        let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
        let scroll_offset = state.preview_scroll_offset;

        // Determine the range of lines to display
        let start_line = scroll_offset;
        let end_line = (start_line + visible_height).min(total_lines);

        // Get the visible content slice
        let visible_content: Vec<_> = if start_line < total_lines {
            state.preview_content[start_line..end_line]
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

    fn render_image_preview(&self, f: &mut Frame, area: Rect, _state: &AppState, path: &std::path::PathBuf) {
        use crate::services::ImagePreview;

        // Try to create image protocol dynamically
        match ImagePreview::create_image_protocol(path) {
            Ok(mut protocol) => {
                // Create the StatefulImage widget
                let image_widget = StatefulImage::new(None);

                // Render the image with the protocol state
                f.render_stateful_widget(image_widget, area, &mut protocol);
            }
            Err(_) => {
                // Fallback to a simple block with error message
                let block = ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("🖼️ Image Preview - Error loading image");
                f.render_widget(block, area);
            }
        }
    }
}
