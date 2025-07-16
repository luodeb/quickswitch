use std::cell::RefCell;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use ratatui_image::picker::Picker;

use crate::{AppState, preview_content::PreviewContent, utils::FileItem};

use super::PreviewGeneratorTrait;

/// Image preview generator
pub struct ImagePreviewGenerator;

impl PreviewGeneratorTrait for ImagePreviewGenerator {
    fn can_handle(&self, file: &FileItem) -> bool {
        file.is_image()
    }

    async fn generate_preview(&self, _state: &AppState, file: &FileItem) -> (String, PreviewContent) {
        let title = format!("🖼️ {}", file.name);

        // Try to load the image
        match image::open(&file.path) {
            Ok(img) => {
                // Create a picker with auto-detected settings from terminal
                let picker = match Picker::from_query_stdio() {
                    Ok(picker) => {
                        // Successfully detected terminal settings - this should give the best quality
                        picker
                    }
                    Err(_) => {
                        // Fallback: use reasonable default font size
                        // Most terminals use roughly 1:2 width:height ratio for font cells
                        Picker::from_fontsize((8, 16))
                    }
                };

                // Create a protocol for the image
                let protocol = picker.new_resize_protocol(img);

                (title, PreviewContent::image(RefCell::new(protocol)))
            }
            Err(e) => {
                let content = vec![
                    Line::from(vec![Span::styled(
                        "Image Load Error".to_string(),
                        Style::default().fg(Color::Red),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        format!("Failed to load image: {e}"),
                        Style::default().fg(Color::Gray),
                    )]),
                ];
                (title, PreviewContent::text(content))
            }
        }
    }
}
