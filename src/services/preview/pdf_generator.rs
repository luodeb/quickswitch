use std::fs;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::PreviewContent;
use crate::utils::FileItem;

use super::{PreviewGeneratorTrait, process_special_characters};

/// PDF preview generator
pub struct PdfPreviewGenerator;

impl PreviewGeneratorTrait for PdfPreviewGenerator {
    fn can_handle(&self, file: &FileItem) -> bool {
        file.is_pdf()
    }

    async fn generate_preview(&self, file: &FileItem) -> (String, PreviewContent) {
        let title = format!("📄 {}", file.name);

        // Try to read the PDF file
        match fs::read(&file.path) {
            Ok(bytes) => {
                // Extract text from PDF using pdf-extract
                match pdf_extract::extract_text_from_mem(&bytes) {
                    Ok(text) => {
                        let lines_count = text.lines().count();
                        let size_info = Line::from(vec![Span::styled(
                            format!("PDF Document - {} lines extracted", lines_count),
                            Style::default().fg(Color::Cyan),
                        )]);

                        let mut lines = vec![size_info];

                        lines.push(Line::from(vec![Span::styled(
                            "─".repeat(50),
                            Style::default().fg(Color::Gray),
                        )]));

                        // Process the extracted text
                        let content_lines: Vec<Line<'static>> = text
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
                    Err(e) => {
                        let content = vec![
                            Line::from(vec![Span::styled(
                                "PDF Processing Error".to_string(),
                                Style::default().fg(Color::Red),
                            )]),
                            Line::from(vec![Span::raw("".to_string())]),
                            Line::from(vec![Span::styled(
                                format!("Failed to extract text from PDF: {e}"),
                                Style::default().fg(Color::Gray),
                            )]),
                            Line::from(vec![Span::raw("".to_string())]),
                            Line::from(vec![Span::styled(
                                "This might be a scanned PDF or contain only images.".to_string(),
                                Style::default().fg(Color::Gray),
                            )]),
                        ];
                        (title, PreviewContent::text(content))
                    }
                }
            }
            Err(e) => {
                let content = vec![
                    Line::from(vec![Span::styled(
                        "PDF Read Error".to_string(),
                        Style::default().fg(Color::Red),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        format!("Failed to read PDF file: {e}"),
                        Style::default().fg(Color::Gray),
                    )]),
                ];
                (title, PreviewContent::text(content))
            }
        }
    }
}
