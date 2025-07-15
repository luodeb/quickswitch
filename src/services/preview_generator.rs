use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use ratatui_image::picker::Picker;
use std::fs;

use crate::{
    app_state::AppState,
    preview_content::{ImageState, PreviewContent},
    utils::FileItem,
};

/// Service for generating preview content for files and directories
pub struct PreviewGenerator;

impl PreviewGenerator {
    /// Generate preview content for a file or directory
    pub fn generate_preview_content(
        state: &AppState,
        file: &FileItem,
    ) -> (String, PreviewContent, Option<ImageState>) {
        if file.is_dir {
            let (title, content) = Self::generate_directory_preview(file);
            (title, PreviewContent::text(content), None)
        } else {
            Self::generate_file_preview(state, file)
        }
    }

    /// Generate preview content for a directory
    fn generate_directory_preview(file: &FileItem) -> (String, Vec<Line<'static>>) {
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
        (title, content)
    }

    /// Generate preview content for Windows drives view
    fn generate_drives_preview() -> (String, Vec<Line<'static>>) {
        let title = "💾 Available Drives".to_string();

        #[cfg(windows)]
        {
            match Self::load_drives() {
                Ok(drives) => {
                    if drives.is_empty() {
                        let content = vec![Line::from(vec![Span::styled(
                            "No drives found".to_string(),
                            Style::default().fg(Color::Gray),
                        )])];
                        (title, content)
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
                        (title, content)
                    }
                }
                Err(e) => {
                    let content = vec![Line::from(vec![Span::styled(
                        format!("Error loading drives: {e}"),
                        Style::default().fg(Color::Red),
                    )])];
                    (title, content)
                }
            }
        }
        #[cfg(not(windows))]
        {
            let content = vec![Line::from(vec![Span::styled(
                "Drive view not available on this platform".to_string(),
                Style::default().fg(Color::Gray),
            )])];
            (title, content)
        }
    }

    /// Load available drives on Windows
    #[cfg(windows)]
    fn load_drives() -> Result<Vec<FileItem>> {
        let mut drives = Vec::new();

        // Try common drive letters and check if they exist
        for letter in 'A'..='Z' {
            let drive_path = format!("{}:\\", letter);
            let path = PathBuf::from(&drive_path);

            // Check if the drive is accessible by trying to read its metadata
            if path.exists() && path.is_dir() {
                drives.push(FileItem {
                    name: drive_path.clone(),
                    path,
                    is_dir: true,
                });
            }
        }

        Ok(drives)
    }

    /// Generate preview content for a file
    fn generate_file_preview(
        state: &AppState,
        file: &FileItem,
    ) -> (String, PreviewContent, Option<ImageState>) {
        // Check if this is an image file
        if file.is_image() {
            return Self::generate_image_preview(state, file);
        }

        // Check if this is a PDF file
        if file.is_pdf() {
            return Self::generate_pdf_preview(file);
        }

        let title = format!("📄 {}", file.name);

        // First check file size to avoid reading large files
        let metadata = match fs::metadata(&file.path) {
            Ok(metadata) => metadata,
            Err(e) => {
                let content = vec![Line::from(vec![Span::styled(
                    format!("Error reading file metadata: {e}"),
                    Style::default().fg(Color::Red),
                )])];
                return (title, PreviewContent::text(content), None);
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
            return (title, PreviewContent::text(content), None);
        }

        // For files under 5MB, try to read and preview content
        let content = match fs::read_to_string(&file.path) {
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
                            Span::raw(Self::process_special_characters(line)),
                        ])
                    })
                    .collect();

                lines.extend(content_lines);

                lines
            }
            Err(_) => {
                // File exists but can't be read as text (likely binary)
                vec![
                    Line::from(vec![Span::styled(
                        "Binary File".to_string(),
                        Style::default().fg(Color::Yellow),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        format!("Size: {file_size} bytes"),
                        Style::default().fg(Color::Gray),
                    )]),
                    Line::from(vec![Span::styled(
                        "Cannot preview binary content".to_string(),
                        Style::default().fg(Color::Gray),
                    )]),
                ]
            }
        };
        (title, PreviewContent::text(content), None)
    }

    /// Generate preview content for a PDF file
    fn generate_pdf_preview(file: &FileItem) -> (String, PreviewContent, Option<ImageState>) {
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
                                    Span::raw(Self::process_special_characters(line)),
                                ])
                            })
                            .collect();

                        lines.extend(content_lines);

                        (title, PreviewContent::text(lines), None)
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
                        (title, PreviewContent::text(content), None)
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
                (title, PreviewContent::text(content), None)
            }
        }
    }

    /// Process special characters in text for better display
    fn process_special_characters(text: &str) -> String {
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

    /// Generate preview content for an image file
    fn generate_image_preview(
        _state: &AppState,
        file: &FileItem,
    ) -> (String, PreviewContent, Option<ImageState>) {
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

                // For now, we'll store the protocol in the image state and use a placeholder in PreviewContent
                let image_state = ImageState { protocol };

                (
                    title,
                    PreviewContent::text(vec![Line::from("🖼️ Image loaded - rendering...")]),
                    Some(image_state),
                )
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
                (title, PreviewContent::text(content), None)
            }
        }
    }
}
