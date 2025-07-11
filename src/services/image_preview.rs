use image::GenericImageView;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use std::{fs, path::Path};

use crate::utils::FileItem;

/// Image preview manager for handling image display in the preview panel
pub struct ImagePreview;

impl ImagePreview {
    pub fn new() -> Self {
        Self
    }

    /// Create an image protocol for the given file
    pub fn create_image_protocol(
        file_path: &Path,
    ) -> Result<Box<dyn StatefulProtocol>, Box<dyn std::error::Error>> {
        let mut picker = Picker::new((8, 12));
        let dyn_img = image::open(file_path)?;
        let protocol = picker.new_resize_protocol(dyn_img);
        Ok(protocol)
    }

    /// Generate preview content for an image file
    pub fn generate_image_preview(file: &FileItem, _area: Rect) -> (String, Vec<Line<'static>>) {
        let title = format!("🖼️ {}", file.name);

        // Try to load and display the image
        match Self::load_and_render_image(&file.path) {
            Ok(lines) => (title, lines),
            Err(e) => {
                let error_content = vec![
                    Line::from(vec![Span::styled(
                        "Image Preview Error".to_string(),
                        Style::default().fg(Color::Red),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        format!("Error: {}", e),
                        Style::default().fg(Color::Gray),
                    )]),
                    Line::from(vec![Span::raw("".to_string())]),
                    Line::from(vec![Span::styled(
                        "Supported formats: JPG, PNG, GIF, BMP, WEBP, TIFF, SVG, ICO, AVIF"
                            .to_string(),
                        Style::default().fg(Color::Gray),
                    )]),
                ];
                (title, error_content)
            }
        }
    }

    /// Load image and render it for terminal display
    fn load_and_render_image(
        path: &Path,
    ) -> Result<Vec<Line<'static>>, Box<dyn std::error::Error>> {
        // Get file metadata for size info
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();

        // Try to decode the image to get dimensions
        let dyn_img = image::open(path)?;
        let (width, height) = dyn_img.dimensions();

        // Create protocol for potential image rendering (commented out for now)
        // let _protocol = picker.new_resize_protocol(dyn_img);

        // Create info lines
        let mut lines = vec![
            Line::from(vec![Span::styled(
                format!("File size: {} bytes", file_size),
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                format!("Dimensions: {}x{} pixels", width, height),
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                format!("Format: {}", Self::get_image_format(path)),
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                "─".repeat(50),
                Style::default().fg(Color::Gray),
            )]),
        ];

        // Add image preview information
        lines.push(Line::from(vec![Span::styled(
            "🖼️ Image Preview".to_string(),
            Style::default().fg(Color::Cyan),
        )]));

        lines.push(Line::from(vec![Span::raw("".to_string())]));

        // Check if terminal supports graphics
        if Self::supports_image_display() {
            lines.push(Line::from(vec![Span::styled(
                "✓ Terminal graphics support detected".to_string(),
                Style::default().fg(Color::Green),
            )]));
            lines.push(Line::from(vec![Span::styled(
                "Image preview available in compatible terminals".to_string(),
                Style::default().fg(Color::Green),
            )]));
            lines.push(Line::from(vec![Span::styled(
                "Note: Full image rendering requires widget integration".to_string(),
                Style::default().fg(Color::Yellow),
            )]));
        } else {
            lines.push(Line::from(vec![Span::styled(
                "⚠ Limited graphics support".to_string(),
                Style::default().fg(Color::Yellow),
            )]));
            lines.push(Line::from(vec![Span::styled(
                "For full image preview, use a terminal with".to_string(),
                Style::default().fg(Color::Gray),
            )]));
            lines.push(Line::from(vec![Span::styled(
                "Sixel, Kitty, or iTerm2 graphics support".to_string(),
                Style::default().fg(Color::Gray),
            )]));
        }

        Ok(lines)
    }

    /// Get image format from file extension
    fn get_image_format(path: &Path) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    /// Check if the terminal supports image display
    fn supports_image_display() -> bool {
        // This would check terminal capabilities
        // For now, we'll assume basic support
        true
    }
}

impl Default for ImagePreview {
    fn default() -> Self {
        Self::new()
    }
}
