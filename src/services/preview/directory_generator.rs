use std::fs;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{AppState, preview_content::PreviewContent, utils::FileItem};

use super::PreviewGeneratorTrait;

/// Directory preview generator
pub struct DirectoryPreviewGenerator;

impl PreviewGeneratorTrait for DirectoryPreviewGenerator {
    fn can_handle(&self, file: &FileItem) -> bool {
        file.is_dir
    }

    fn generate_preview(&self, _state: &AppState, file: &FileItem) -> (String, PreviewContent) {
        // Special handling for Windows drives view
        if file.path.to_string_lossy() == "DRIVES:" {
            return Self::generate_drives_preview();
        }

        let title = format!("📁 {}", file.name);
        let content = match fs::read_dir(&file.path) {
            Ok(entries) => {
                let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                items.sort_by(|a, b| {
                    let a_is_dir = a.path().is_dir();
                    let b_is_dir = b.path().is_dir();
                    match (a_is_dir, b_is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.file_name().cmp(&b.file_name()),
                    }
                });

                let mut preview_content: Vec<Line<'static>> = items
                    .iter()
                    .map(|entry| {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        let is_dir = entry.path().is_dir();
                        let icon = if is_dir { "📁" } else { "📄" };
                        let style = if is_dir {
                            Style::default().fg(Color::Cyan)
                        } else {
                            Style::default()
                        };

                        Line::from(vec![
                            Span::raw(icon.to_string()),
                            Span::raw(" ".to_string()),
                            Span::styled(name, style),
                        ])
                    })
                    .collect();

                if preview_content.is_empty() {
                    preview_content.push(Line::from(vec![Span::styled(
                        "Empty directory".to_string(),
                        Style::default().fg(Color::Gray),
                    )]));
                }

                preview_content
            }
            Err(e) => {
                vec![Line::from(vec![Span::styled(
                    format!("Error reading directory: {e}"),
                    Style::default().fg(Color::Red),
                )])]
            }
        };
        (title, PreviewContent::text(content))
    }
}

impl DirectoryPreviewGenerator {
    /// Generate preview content for Windows drives view
    fn generate_drives_preview() -> (String, PreviewContent) {
        let title = "💾 Available Drives".to_string();

        #[cfg(windows)]
        {
            use crate::services::FilesystemService;
            match FilesystemService::load_drives() {
                Ok(drives) => {
                    if drives.is_empty() {
                        let content = vec![Line::from(vec![Span::styled(
                            "No drives found".to_string(),
                            Style::default().fg(Color::Gray),
                        )])];
                        (title, PreviewContent::text(content))
                    } else {
                        let content: Vec<Line<'static>> = drives
                            .iter()
                            .map(|drive| {
                                Line::from(vec![
                                    Span::raw("💾 ".to_string()),
                                    Span::styled(
                                        drive.name.clone(),
                                        Style::default().fg(Color::Cyan),
                                    ),
                                ])
                            })
                            .collect();
                        (title, PreviewContent::text(content))
                    }
                }
                Err(e) => {
                    let content = vec![Line::from(vec![Span::styled(
                        format!("Error loading drives: {e}"),
                        Style::default().fg(Color::Red),
                    )])];
                    (title, PreviewContent::text(content))
                }
            }
        }
        #[cfg(not(windows))]
        {
            let content = vec![Line::from(vec![Span::styled(
                "Drive view not available on this platform".to_string(),
                Style::default().fg(Color::Gray),
            )])];
            (title, PreviewContent::text(content))
        }
    }
}
