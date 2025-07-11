use anyhow::Result;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::{fs, path::PathBuf};

use crate::utils::FileItem;

/// Service for filesystem operations
pub struct FilesystemService;

impl FilesystemService {
    /// Load directory contents and return sorted file list
    pub fn load_directory(current_dir: &PathBuf) -> Result<Vec<FileItem>> {
        let mut files = Vec::new();

        // Check if we're at Windows drives view and should show drives
        if Self::should_show_drives(current_dir) {
            return Self::load_drives();
        }

        // 添加当前目录
        files.push(FileItem {
            name: ".".to_string(),
            path: current_dir.clone(),
            is_dir: true,
        });

        let entries = fs::read_dir(current_dir)?;
        let mut items: Vec<FileItem> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                Some(FileItem { name, path, is_dir })
            })
            .collect();

        items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        files.extend(items);
        Ok(files)
    }

    /// Check if we should show drives instead of directory contents
    fn should_show_drives(current_dir: &PathBuf) -> bool {
        #[cfg(windows)]
        {
            // On Windows, only show drives when we're at the special "DRIVES:" path
            current_dir.to_string_lossy() == "DRIVES:"
        }
        #[cfg(not(windows))]
        {
            let _ = current_dir; // Suppress unused variable warning
            false
        }
    }

    /// Load available drives on Windows
    pub fn load_drives() -> Result<Vec<FileItem>> {
        #[cfg(windows)]
        {
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
        #[cfg(not(windows))]
        {
            Ok(Vec::new())
        }
    }

    /// Generate preview content for a file or directory
    pub fn generate_preview_content(file: &FileItem) -> (String, Vec<Line<'static>>) {
        if file.is_dir {
            Self::generate_directory_preview(file)
        } else {
            Self::generate_file_preview(file)
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

    /// Generate preview content for a file
    fn generate_file_preview(file: &FileItem) -> (String, Vec<Line<'static>>) {
        let title = format!("📄 {}", file.name);

        // First check file size to avoid reading large files
        let metadata = match fs::metadata(&file.path) {
            Ok(metadata) => metadata,
            Err(e) => {
                let content = vec![Line::from(vec![Span::styled(
                    format!("Error reading file metadata: {e}"),
                    Style::default().fg(Color::Red),
                )])];
                return (title, content);
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
            return (title, content);
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
                            Span::raw(line.to_string()),
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
                        format!("Size: {} bytes", file_size),
                        Style::default().fg(Color::Gray),
                    )]),
                    Line::from(vec![Span::styled(
                        "Cannot preview binary content".to_string(),
                        Style::default().fg(Color::Gray),
                    )]),
                ]
            }
        };
        (title, content)
    }
}
