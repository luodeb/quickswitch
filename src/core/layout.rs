use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Layout manager for handling UI area calculations and management
#[derive(Debug, Clone, Default)]
pub struct LayoutManager {
    /// The entire terminal area
    pub terminal_area: Rect,
    /// Search box area at the top
    pub search_area: Rect,
    /// Main content area (below search box)
    pub main_area: Rect,
    /// Left panel area (file list or history)
    pub left_area: Rect,
    /// Right panel area (preview or help)
    pub right_area: Rect,
    /// Whether the layout has been initialized
    initialized: bool,
}

impl LayoutManager {
    /// Create a new layout manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize or update the layout based on terminal size
    pub fn update_layout(&mut self, terminal_size: Rect) {
        self.terminal_area = terminal_size;

        // Split vertically: search box (3 lines) + main content
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(self.terminal_area);

        self.search_area = vertical_chunks[0];
        self.main_area = vertical_chunks[1];

        // Split main area horizontally: left panel (50%) + right panel (50%)
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(self.main_area);

        self.left_area = horizontal_chunks[0];
        self.right_area = horizontal_chunks[1];

        self.initialized = true;
    }

    /// Update layout with custom constraints for left/right panels
    pub fn update_layout_with_constraints(
        &mut self,
        terminal_size: Rect,
        left_constraint: Constraint,
        right_constraint: Constraint,
    ) {
        self.terminal_area = terminal_size;

        // Split vertically: search box (3 lines) + main content
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(self.terminal_area);

        self.search_area = vertical_chunks[0];
        self.main_area = vertical_chunks[1];

        // Split main area horizontally with custom constraints
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([left_constraint, right_constraint])
            .split(self.main_area);

        self.left_area = horizontal_chunks[0];
        self.right_area = horizontal_chunks[1];

        self.initialized = true;
    }

    /// Check if the layout has been initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the terminal area
    pub fn get_terminal_area(&self) -> Rect {
        self.terminal_area
    }

    /// Get the search area
    pub fn get_search_area(&self) -> Rect {
        self.search_area
    }

    /// Get the main content area
    pub fn get_main_area(&self) -> Rect {
        self.main_area
    }

    /// Get the left panel area
    pub fn get_left_area(&self) -> Rect {
        self.left_area
    }

    /// Get the right panel area
    pub fn get_right_area(&self) -> Rect {
        self.right_area
    }

    /// Check if a point (x, y) is within the left area
    pub fn is_in_left_area(&self, x: u16, y: u16) -> bool {
        x >= self.left_area.x
            && x < self.left_area.x + self.left_area.width
            && y >= self.left_area.y
            && y < self.left_area.y + self.left_area.height
    }

    /// Check if a point (x, y) is within the right area
    pub fn is_in_right_area(&self, x: u16, y: u16) -> bool {
        x >= self.right_area.x
            && x < self.right_area.x + self.right_area.width
            && y >= self.right_area.y
            && y < self.right_area.y + self.right_area.height
    }

    /// Check if a point (x, y) is within the search area
    pub fn is_in_search_area(&self, x: u16, y: u16) -> bool {
        x >= self.search_area.x
            && x < self.search_area.x + self.search_area.width
            && y >= self.search_area.y
            && y < self.search_area.y + self.search_area.height
    }

    /// Get the terminal dimensions (width, height)
    pub fn get_terminal_size(&self) -> (u16, u16) {
        (self.terminal_area.width, self.terminal_area.height)
    }

    /// Get the left panel dimensions (width, height)
    pub fn get_left_panel_size(&self) -> (u16, u16) {
        (self.left_area.width, self.left_area.height)
    }

    /// Get the right panel dimensions (width, height)
    pub fn get_right_panel_size(&self) -> (u16, u16) {
        (self.right_area.width, self.right_area.height)
    }

    /// Calculate the visible height for content (excluding borders)
    pub fn get_content_height(&self, area: Rect) -> usize {
        area.height.saturating_sub(2) as usize // Account for top and bottom borders
    }

    /// Calculate the visible width for content (excluding borders)
    pub fn get_content_width(&self, area: Rect) -> usize {
        area.width.saturating_sub(2) as usize // Account for left and right borders
    }

    /// Get the visible content height for the left panel
    pub fn get_left_content_height(&self) -> usize {
        self.get_content_height(self.left_area)
    }

    /// Get the visible content height for the right panel
    pub fn get_right_content_height(&self) -> usize {
        self.get_content_height(self.right_area)
    }

    /// Get the visible content width for the left panel
    pub fn get_left_content_width(&self) -> usize {
        self.get_content_width(self.left_area)
    }

    /// Get the visible content width for the right panel
    pub fn get_right_content_width(&self) -> usize {
        self.get_content_width(self.right_area)
    }

    /// Check if the layout needs to be updated based on new terminal size
    pub fn needs_update(&self, new_terminal_size: Rect) -> bool {
        !self.initialized || self.terminal_area != new_terminal_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_manager_initialization() {
        let mut layout = LayoutManager::new();
        assert!(!layout.is_initialized());

        let terminal_size = Rect::new(0, 0, 100, 50);
        layout.update_layout(terminal_size);

        assert!(layout.is_initialized());
        assert_eq!(layout.get_terminal_area(), terminal_size);
        assert_eq!(layout.get_search_area().height, 3);
        assert_eq!(layout.get_main_area().height, 47);
    }

    #[test]
    fn test_point_in_area_detection() {
        let mut layout = LayoutManager::new();
        let terminal_size = Rect::new(0, 0, 100, 50);
        layout.update_layout(terminal_size);

        // Test left area detection
        assert!(layout.is_in_left_area(25, 25)); // Should be in left area
        assert!(!layout.is_in_left_area(75, 25)); // Should be in right area

        // Test right area detection
        assert!(layout.is_in_right_area(75, 25)); // Should be in right area
        assert!(!layout.is_in_right_area(25, 25)); // Should be in left area
    }

    #[test]
    fn test_content_size_calculation() {
        let mut layout = LayoutManager::new();
        let terminal_size = Rect::new(0, 0, 100, 50);
        layout.update_layout(terminal_size);

        // Content height should account for borders
        let left_content_height = layout.get_left_content_height();
        let right_content_height = layout.get_right_content_height();

        assert!(left_content_height > 0);
        assert!(right_content_height > 0);
        assert_eq!(left_content_height, right_content_height);
    }
}
