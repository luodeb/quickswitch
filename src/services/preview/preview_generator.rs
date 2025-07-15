use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::fs;

use crate::{app_state::AppState, preview_content::PreviewContent, utils::FileItem};

/// Trait for preview generators
pub trait PreviewGeneratorTrait {
    /// Generate preview content for a file
    fn generate_preview(&self, state: &AppState, file: &FileItem) -> (String, PreviewContent);

    /// Check if this generator can handle the given file
    fn can_handle(&self, file: &FileItem) -> bool;
}

use super::{
    DirectoryPreviewGenerator, ImagePreviewGenerator, PdfPreviewGenerator, TextPreviewGenerator,
};

/// Main service for generating preview content for files and directories
pub struct PreviewGenerator;

impl PreviewGenerator {
    /// Generate preview content for a file or directory
    pub fn generate_preview_content(state: &AppState, file: &FileItem) -> (String, PreviewContent) {
        // Try different file preview generators in order
        let generators: Vec<Box<dyn PreviewGeneratorTrait>> = vec![
            Box::new(DirectoryPreviewGenerator),
            Box::new(ImagePreviewGenerator),
            Box::new(PdfPreviewGenerator),
            Box::new(TextPreviewGenerator),
        ];

        for generator in generators {
            if generator.can_handle(file) {
                return generator.generate_preview(state, file);
            }
        }

        // Fallback to binary file preview
        BinaryPreviewGenerator.generate_preview(state, file)
    }
}

/// Process special characters in text for better display
pub fn process_special_characters(text: &str) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        match ch {
            '\t' => {
                // Replace tab with visible representation and spaces
                result.push_str("→   "); // Arrow symbol followed by 3 spaces for tab width
            }
            '\r' => {
                // Replace carriage return with visible representation
                result.push_str("\\r");
            }
            '\0' => {
                // Replace null character with visible representation
                result.push_str("\\0");
            }
            c if c.is_control() && c != '\n' => {
                // Replace other control characters with their escape sequence
                result.push_str(&format!("\\x{:02x}", c as u8));
            }
            c => {
                // Keep normal characters as-is
                result.push(c);
            }
        }
    }

    result
}

/// Binary file preview generator (fallback)
pub struct BinaryPreviewGenerator;

impl PreviewGeneratorTrait for BinaryPreviewGenerator {
    fn can_handle(&self, _file: &FileItem) -> bool {
        // This is a fallback generator, so it can handle any file
        true
    }

    fn generate_preview(&self, _state: &AppState, file: &FileItem) -> (String, PreviewContent) {
        let title = format!("📄 {}", file.name);

        // Get file metadata
        let metadata = match fs::metadata(&file.path) {
            Ok(metadata) => metadata,
            Err(e) => {
                let content = vec![Line::from(vec![Span::styled(
                    format!("Error reading file metadata: {e}"),
                    Style::default().fg(Color::Red),
                )])];
                return (title, PreviewContent::text(content));
            }
        };

        let file_size = metadata.len();

        let content = vec![
            Line::from(vec![Span::styled(
                "Binary File".to_string(),
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(vec![Span::raw("".to_string())]),
            Line::from(vec![Span::styled(
                format!("Size: {} bytes", file_size),
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                "Cannot preview binary content".to_string(),
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::raw("".to_string())]),
            Line::from(vec![Span::styled(
                "File type: Binary/Unknown".to_string(),
                Style::default().fg(Color::Cyan),
            )]),
        ];

        (title, PreviewContent::text(content))
    }
}
