use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
use std::sync::{Arc, RwLock};

use crate::preview_content::PreviewContent;

/// Global preview state that can be safely accessed from multiple threads
#[derive(Debug, Clone)]
pub struct PreviewState {
    pub content: PreviewContent,
    pub title: String,
    pub scroll_offset: usize,
}

impl Default for PreviewState {
    fn default() -> Self {
        Self {
            content: PreviewContent::text(vec![Line::from(vec![Span::styled(
                "No file selected".to_string(),
                Style::default().fg(Color::Gray),
            )])]),
            title: "Preview".to_string(),
            scroll_offset: 0,
        }
    }
}

/// Global preview state manager with thread-safe access
pub struct GlobalPreviewState {
    state: Arc<RwLock<PreviewState>>,
}

impl GlobalPreviewState {
    /// Create a new global preview state
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(PreviewState::default())),
        }
    }

    /// Get a copy of the current preview state
    pub fn get_state(&self) -> PreviewState {
        self.state.read().unwrap().clone()
    }

    /// Update the preview content and title
    pub fn update_preview(&self, title: String, content: PreviewContent) {
        let mut state = self.state.write().unwrap();
        state.title = title;
        state.content = content;
        state.scroll_offset = 0; // Reset scroll when content changes
    }

    /// Clear the preview content
    pub fn clear_preview(&self) {
        let mut state = self.state.write().unwrap();
        state.title = "Preview".to_string();
        state.content = PreviewContent::text(vec![Line::from(vec![Span::styled(
            "No file selected".to_string(),
            Style::default().fg(Color::Gray),
        )])]);
        state.scroll_offset = 0;
    }

    /// Get the current preview title
    pub fn get_title(&self) -> String {
        self.state.read().unwrap().title.clone()
    }

    /// Get the current preview content
    pub fn get_content(&self) -> PreviewContent {
        self.state.read().unwrap().content.clone()
    }

    /// Get the current scroll offset
    pub fn get_scroll_offset(&self) -> usize {
        self.state.read().unwrap().scroll_offset
    }

    /// Set the scroll offset
    pub fn set_scroll_offset(&self, offset: usize) {
        let mut state = self.state.write().unwrap();
        state.scroll_offset = offset;
    }

    /// Scroll preview content up by one line
    pub fn scroll_up(&self) -> bool {
        let mut state = self.state.write().unwrap();
        if state.scroll_offset > 0 {
            state.scroll_offset -= 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by one line
    pub fn scroll_down(&self) -> bool {
        let mut state = self.state.write().unwrap();
        if state.scroll_offset + 1 < state.content.len() {
            state.scroll_offset += 1;
            true
        } else {
            false
        }
    }

    /// Scroll preview content up by half screen (page up)
    pub fn scroll_page_up(&self, visible_height: usize) -> bool {
        let mut state = self.state.write().unwrap();
        let half_screen = (visible_height / 2).max(1);
        let new_offset = state.scroll_offset.saturating_sub(half_screen);
        if new_offset != state.scroll_offset {
            state.scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Scroll preview content down by half screen (page down)
    pub fn scroll_page_down(&self, visible_height: usize) -> bool {
        let mut state = self.state.write().unwrap();
        let half_screen = (visible_height / 2).max(1);
        let max_offset = state.content.len().saturating_sub(visible_height);
        let new_offset = (state.scroll_offset + half_screen).min(max_offset);
        if new_offset != state.scroll_offset {
            state.scroll_offset = new_offset;
            true
        } else {
            false
        }
    }

    /// Reset scroll position to top
    pub fn reset_scroll(&self) {
        let mut state = self.state.write().unwrap();
        state.scroll_offset = 0;
    }
}

impl Default for GlobalPreviewState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global instance of the preview state
pub static GLOBAL_PREVIEW_STATE: Lazy<GlobalPreviewState> = Lazy::new(|| GlobalPreviewState::new());

/// Convenience functions for accessing the global preview state
impl GlobalPreviewState {
    /// Get the global preview state instance
    pub fn instance() -> &'static GlobalPreviewState {
        &GLOBAL_PREVIEW_STATE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{
        style::Style,
        text::{Line, Span},
    };

    #[test]
    fn test_global_preview_state_basic_operations() {
        let global_state = GlobalPreviewState::new();

        // Test initial state
        let state = global_state.get_state();
        assert_eq!(state.title, "Preview");
        assert_eq!(state.scroll_offset, 0);

        // Test updating preview
        let test_lines = vec![
            Line::from(vec![Span::styled("Line 1", Style::default())]),
            Line::from(vec![Span::styled("Line 2", Style::default())]),
        ];
        let test_content = PreviewContent::text(test_lines);

        global_state.update_preview("Test Title".to_string(), test_content);

        let updated_state = global_state.get_state();
        assert_eq!(updated_state.title, "Test Title");
        assert_eq!(updated_state.scroll_offset, 0); // Should reset on update

        // Test scrolling
        assert!(global_state.scroll_down());
        assert_eq!(global_state.get_scroll_offset(), 1);

        assert!(global_state.scroll_up());
        assert_eq!(global_state.get_scroll_offset(), 0);

        // Test clear
        global_state.clear_preview();
        let cleared_state = global_state.get_state();
        assert_eq!(cleared_state.title, "Preview");
        assert_eq!(cleared_state.scroll_offset, 0);
    }

    #[test]
    fn test_global_preview_state_singleton() {
        let instance1 = GlobalPreviewState::instance();
        let instance2 = GlobalPreviewState::instance();

        // Both should point to the same instance
        instance1.update_preview("Test".to_string(), PreviewContent::text(vec![]));
        assert_eq!(instance2.get_title(), "Test");
    }

    #[test]
    fn test_scroll_bounds() {
        let global_state = GlobalPreviewState::new();

        // Test scrolling up when at top
        assert!(!global_state.scroll_up());
        assert_eq!(global_state.get_scroll_offset(), 0);

        // Add content with multiple lines
        let test_lines = vec![
            Line::from("Line 1"),
            Line::from("Line 2"),
            Line::from("Line 3"),
        ];
        global_state.update_preview("Test".to_string(), PreviewContent::text(test_lines));

        // Test scrolling down beyond content
        assert!(global_state.scroll_down());
        assert!(global_state.scroll_down());
        assert!(!global_state.scroll_down()); // Should fail when at bottom
    }
}
