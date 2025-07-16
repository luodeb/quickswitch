use std::sync::Arc;

use tokio::sync::Mutex;

use ratatui::text::Line;
use ratatui_image::protocol::StatefulProtocol;

/// Enum representing different types of preview content
pub enum PreviewContent {
    /// Text content with lines for display
    Text(Vec<Line<'static>>),
    /// Image content with protocol for rendering
    Image(Arc<Mutex<StatefulProtocol>>),
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
    pub fn image(protocol: Arc<Mutex<StatefulProtocol>>) -> Self {
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
    pub fn as_image(&self) -> Option<&Arc<Mutex<StatefulProtocol>>> {
        match self {
            Self::Text(_) => None,
            Self::Image(protocol) => Some(protocol),
        }
    }

    /// Get mutable image protocol if this is image content
    pub fn as_image_mut(&mut self) -> Option<&mut Arc<Mutex<StatefulProtocol>>> {
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
            Self::Image(image) => Self::Image(image.clone()),
        }
    }
}

impl Default for PreviewContent {
    fn default() -> Self {
        Self::Text(Vec::new())
    }
}

impl std::fmt::Debug for PreviewContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(lines) => f
                .debug_tuple("Text")
                .field(&format!("{} lines", lines.len()))
                .finish(),
            Self::Image(_) => f.debug_tuple("Image").field(&"StatefulProtocol").finish(),
        }
    }
}
