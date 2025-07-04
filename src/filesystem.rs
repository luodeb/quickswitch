use anyhow::Result;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::{fs, path::PathBuf};

use crate::models::FileItem;

pub fn load_directory(current_dir: &PathBuf) -> Result<Vec<FileItem>> {
    let mut files = Vec::new();

    if let Some(parent) = current_dir.parent() {
        files.push(FileItem {
            name: "..".to_string(),
            path: parent.to_path_buf(),
            is_dir: true,
        });
    }

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

pub fn generate_preview_content(file: &FileItem) -> (String, Vec<Line<'static>>) {
    if file.is_dir {
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
                    .take(100)
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

                if items.len() > 100 {
                    preview_content.push(Line::from(vec![Span::styled(
                        format!("... and {} more items", items.len() - 100),
                        Style::default().fg(Color::Gray),
                    )]));
                }

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
                    format!("Error reading directory: {}", e),
                    Style::default().fg(Color::Red),
                )])]
            }
        };
        (title, content)
    } else {
        let title = format!("📄 {}", file.name);
        let content = match fs::read_to_string(&file.path) {
            Ok(content) => {
                let size_info = Line::from(vec![Span::styled(
                    format!(
                        "Size: {} bytes, {} lines\n",
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
                    .take(100)
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

                if content.lines().count() > 100 {
                    lines.push(Line::from(vec![Span::styled(
                        format!("... ({} more lines)", content.lines().count() - 100),
                        Style::default().fg(Color::Gray),
                    )]));
                }

                lines
            }
            Err(_) => match fs::metadata(&file.path) {
                Ok(metadata) => {
                    vec![
                        Line::from(vec![Span::styled(
                            "Binary File".to_string(),
                            Style::default().fg(Color::Yellow),
                        )]),
                        Line::from(vec![Span::raw("".to_string())]),
                        Line::from(vec![Span::styled(
                            format!("Size: {} bytes", metadata.len()),
                            Style::default().fg(Color::Gray),
                        )]),
                        Line::from(vec![Span::styled(
                            "Cannot preview binary content".to_string(),
                            Style::default().fg(Color::Gray),
                        )]),
                    ]
                }
                Err(e) => {
                    vec![Line::from(vec![Span::styled(
                        format!("Error reading file: {}", e),
                        Style::default().fg(Color::Red),
                    )])]
                }
            },
        };
        (title, content)
    }
}
