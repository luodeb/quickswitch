use std::fs;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{AppState, preview_content::PreviewContent, utils::FileItem};

use super::{PreviewGeneratorTrait, process_special_characters};

/// Text preview generator
pub struct TextPreviewGenerator;

impl PreviewGeneratorTrait for TextPreviewGenerator {
    fn can_handle(&self, file: &FileItem) -> bool {
        // Handle any file that's not an image or PDF and can be read as text
        fs::read_to_string(&file.path).is_ok()
    }

    async fn generate_preview(&self, _state: &AppState, file: &FileItem) -> (String, PreviewContent) {
        let title = format!("📄 {}", file.name);

        // First check file size to avoid reading large files
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
        const MAX_PREVIEW_SIZE: u64 = 5 * 1024 * 1024; // 5MB

        // If file is too large, only show basic information
        if file_size > MAX_PREVIEW_SIZE {
            let content = vec![
                Line::from(vec![Span::styled(
                    "Large File".to_string(),
                    Style::default().fg(Color::Yellow),
                )]),
                Line::from(vec![Span::raw("".to_string())]),
                Line::from(vec![Span::styled(
                    format!(
                        "Size: {} bytes ({:.2} MB)",
                        file_size,
                        file_size as f64 / 1024.0 / 1024.0
                    ),
                    Style::default().fg(Color::Gray),
                )]),
                Line::from(vec![Span::styled(
                    "File too large for preview (>5MB)".to_string(),
                    Style::default().fg(Color::Gray),
                )]),
                Line::from(vec![Span::raw("".to_string())]),
                Line::from(vec![Span::styled(
                    "Basic file information:".to_string(),
                    Style::default().fg(Color::Cyan),
                )]),
            ];
            return (title, PreviewContent::text(content));
        }

        // For files under 5MB, try to read and preview content
        match fs::read_to_string(&file.path) {
            Ok(content) => {
                let size_info = Line::from(vec![Span::styled(
                    format!(
                        "Size: {} bytes, {} lines",
                        content.len(),
                        content.lines().count()
                    ),
                    Style::default().fg(Color::Gray),
                )]);

                let mut lines = vec![size_info];

                lines.push(Line::from(vec![Span::styled(
                    "─".repeat(50),
                    Style::default().fg(Color::Gray),
                )]));

                let content_lines: Vec<Line<'static>> = content
                    .lines()
                    .enumerate()
                    .map(|(i, line)| {
                        Line::from(vec![
                            Span::styled(
                                format!("{:3} ", i + 1),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::raw(process_special_characters(line)),
                        ])
                    })
                    .collect();

                lines.extend(content_lines);

                (title, PreviewContent::text(lines))
            }
            Err(_) => {
                // File exists but can't be read as text (likely binary)
                // This should be handled by BinaryPreviewGenerator, but as fallback
                let content = vec![
                    Line::from(vec![Span::styled(
                        "Text Read Error".to_string(),
                        Style::default().fg(Color::Red),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        format!("Size: {file_size} bytes"),
                        Style::default().fg(Color::Gray),
                    )]),
                    Line::from(vec![Span::styled(
                        "Cannot read as text".to_string(),
                        Style::default().fg(Color::Gray),
                    )]),
                ];
                (title, PreviewContent::text(content))
            }
        }
    }
}
