use std::cell::RefCell;

use ratatui::text::Line;
use ratatui_image::protocol::StatefulProtocol;

/// Enum representing different types of preview content
pub enum PreviewContent {
    /// Text content with lines for display
    Text(Vec<Line<'static>>),
    /// Image content with protocol for rendering
    Image(RefCell<StatefulProtocol>),
}

/// Image state that can be stored in AppState
pub struct ImageState {
    pub protocol: StatefulProtocol,
}

impl PreviewContent {
    /// Create text preview content
    pub fn text(lines: Vec<Line<'static>>) -> Self {
        Self::Text(lines)
    }

    /// Create image preview content
    pub fn image(protocol: RefCell<StatefulProtocol>) -> Self {
        Self::Image(protocol)
    }

    /// Check if this is text content
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Check if this is image content
    pub fn is_image(&self) -> bool {
        matches!(self, Self::Image(_))
    }

    /// Get text lines if this is text content
    pub fn as_text(&self) -> Option<&Vec<Line<'static>>> {
        match self {
            Self::Text(lines) => Some(lines),
            Self::Image(_) => None,
        }
    }

    /// Get image protocol if this is image content
    pub fn as_image(&self) -> Option<&RefCell<StatefulProtocol>> {
        match self {
            Self::Text(_) => None,
            Self::Image(protocol) => Some(protocol),
        }
    }

    /// Get mutable image protocol if this is image content
    pub fn as_image_mut(&mut self) -> Option<&mut RefCell<StatefulProtocol>> {
        match self {
            Self::Text(_) => None,
            Self::Image(protocol) => Some(protocol),
        }
    }
}

impl PreviewContent {
    /// Get the length of content (number of lines for text, 1 for image)
    pub fn len(&self) -> usize {
        match self {
            Self::Text(lines) => lines.len(),
            Self::Image(_) => 1, // Images take up the full area
        }
    }

    /// Check if content is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Text(lines) => lines.is_empty(),
            Self::Image(_) => false, // Images are never considered empty
        }
    }
}

impl Clone for PreviewContent {
    fn clone(&self) -> Self {
        match self {
            Self::Text(lines) => Self::Text(lines.clone()),
            Self::Image(_) => {
                // For images, we'll return an empty text content as a fallback
                // since StatefulProtocol doesn't implement Clone
                Self::Text(vec![Line::from("Image content (clone not supported)")])
            }
        }
    }
}

impl Default for PreviewContent {
    fn default() -> Self {
        Self::Text(Vec::new())
    }
}
