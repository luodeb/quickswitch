use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::{
    FileItem,
    app::App,
    models::{AppMode, DisplayItem},
    modes::ModeAction,
    services::{DataProvider, PreviewManager, create_data_provider},
};

/// Unified input dispatcher for handling all user interactions
/// This centralizes key and mouse event handling, reducing duplication across modes
pub struct InputDispatcher;

impl InputDispatcher {
    /// Handle keyboard input uniformly across all modes
    pub fn handle_key_event(
        app: &mut App,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Handle exit keys first (highest priority)
        if let Some(action) = Self::handle_exit_keys(app, key, current_mode) {
            return Ok(action);
        }

        // Handle mode switch keys
        if let Some(action) = Self::handle_mode_switch_keys(app, key, current_mode) {
            return Ok(action);
        }

        // Handle navigation keys (unified for all modes)
        if let Some(action) = Self::handle_navigation_keys(app, key, current_mode)? {
            return Ok(action);
        }

        // Handle mode-specific keys
        Self::handle_mode_specific_keys(app, key, current_mode)
    }

    /// Handle mouse input uniformly across all modes
    pub fn handle_mouse_event(
        app: &mut App,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        match mouse.kind {
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                Self::handle_scroll_navigation(app, mouse, left_area, right_area, current_mode)
            }
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                Self::handle_left_click(app, mouse, left_area, current_mode)
            }
            _ => Ok(ModeAction::Stay),
        }
    }

    /// Handle exit keys (Esc, Enter) - unified across all modes
    fn handle_exit_keys(app: &mut App, key: KeyCode, current_mode: &AppMode) -> Option<ModeAction> {
        match key {
            KeyCode::Esc => {
                // If searching, exit search mode but keep search input and results
                if app.state.is_searching {
                    app.state.is_searching = false;
                    // Don't clear search_input - keep the search results visible
                    // app.update_filter() is not needed since we're keeping the same filter
                    PreviewManager::update_preview_from_selection(app);
                    Some(ModeAction::Stay)
                } else if current_mode == &AppMode::Normal {
                    // In normal mode, Esc exits the application
                    Some(ModeAction::Exit(None))
                } else {
                    // In other modes, Esc returns to normal mode
                    Some(ModeAction::Switch(AppMode::Normal))
                }
            }
            KeyCode::Enter => {
                // Handle selection and exit using unified data provider
                let provider = create_data_provider(current_mode);
                if let Some(item) = provider.get_selected_item(app) {
                    match item {
                        DisplayItem::File(file) => Some(ModeAction::Exit(Some(file))),
                        DisplayItem::HistoryPath(path) => {
                            let file_item = FileItem::from_path(&path);
                            Some(ModeAction::Exit(Some(file_item)))
                        }
                    }
                } else {
                    Some(ModeAction::Stay)
                }
            }
            _ => None,
        }
    }

    /// Handle mode switching keys - unified across all modes
    fn handle_mode_switch_keys(
        app: &mut App,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Option<ModeAction> {
        match key {
            KeyCode::Char('/') => {
                // Enable search functionality in normal and history modes
                if matches!(current_mode, AppMode::Normal | AppMode::History)
                    && !app.state.is_searching
                {
                    app.state.is_searching = true;
                    Some(ModeAction::Stay)
                } else {
                    None
                }
            }
            KeyCode::Char('v') if !app.state.is_searching => {
                if current_mode != &AppMode::History {
                    Some(ModeAction::Switch(AppMode::History))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Handle navigation keys - unified using data providers
    fn handle_navigation_keys(
        app: &mut App,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Result<Option<ModeAction>> {
        let provider = create_data_provider(current_mode);

        match key {
            KeyCode::Up => {
                provider.navigate_up(app);
                Self::update_preview_if_needed(app, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Down => {
                provider.navigate_down(app);
                Self::update_preview_if_needed(app, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Right => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_into_directory(app)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::Left => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_to_parent(app)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            // hjkl keys only work when not searching
            KeyCode::Char('k') if !app.state.is_searching => {
                provider.navigate_up(app);
                Self::update_preview_if_needed(app, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Char('j') if !app.state.is_searching => {
                provider.navigate_down(app);
                Self::update_preview_if_needed(app, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Char('l') if !app.state.is_searching => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_into_directory(app)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::Char('h') if !app.state.is_searching => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_to_parent(app)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::PageUp | KeyCode::PageDown => {
                Self::handle_preview_navigation(app, key);
                Ok(Some(ModeAction::Stay))
            }
            _ => Ok(None),
        }
    }

    /// Handle mode-specific keys that don't fit into common patterns
    fn handle_mode_specific_keys(
        app: &mut App,
        key: KeyCode,
        _current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Handle search input when in search mode
        if app.state.is_searching {
            Self::handle_search_keys(app, key)
        } else {
            Ok(ModeAction::Stay)
        }
    }

    /// Handle search mode specific keys
    fn handle_search_keys(app: &mut App, key: KeyCode) -> Result<ModeAction> {
        match key {
            KeyCode::Char(c) => {
                app.state.search_input.push(c);
                app.update_filter();
                PreviewManager::update_preview_from_selection(app);
                Ok(ModeAction::Stay)
            }
            KeyCode::Backspace => {
                app.state.search_input.pop();
                app.update_filter();
                PreviewManager::update_preview_from_selection(app);
                Ok(ModeAction::Stay)
            }
            _ => Ok(ModeAction::Stay),
        }
    }

    /// Handle preview navigation (Page Up/Down)
    fn handle_preview_navigation(app: &mut App, key: KeyCode) {
        match key {
            KeyCode::PageUp => {
                PreviewManager::scroll_preview_up(app);
            }
            KeyCode::PageDown => {
                PreviewManager::scroll_preview_down(app);
            }
            _ => {}
        }
    }

    /// Update preview if the data provider supports it
    fn update_preview_if_needed(app: &mut App, provider: &dyn DataProvider) {
        if let Some(path) = provider.get_preview_path(app) {
            PreviewManager::update_preview_for_path(app, &path);
        }
    }

    /// Handle scroll navigation using unified data providers
    fn handle_scroll_navigation(
        app: &mut App,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        let is_scroll_up = matches!(mouse.kind, MouseEventKind::ScrollUp);

        // Check if mouse is in left area (file/history list) or right area (preview)
        if mouse.column >= left_area.x && mouse.column < left_area.x + left_area.width {
            // Mouse is in left panel - scroll list using unified provider
            let provider = create_data_provider(current_mode);
            if is_scroll_up {
                provider.navigate_up(app);
            } else {
                provider.navigate_down(app);
            }
            Self::update_preview_if_needed(app, &*provider);
        } else if mouse.column >= right_area.x && mouse.column < right_area.x + right_area.width {
            // Mouse is in right panel - scroll preview content
            if is_scroll_up {
                PreviewManager::scroll_preview_up(app);
            } else {
                PreviewManager::scroll_preview_down(app);
            }
        }

        Ok(ModeAction::Stay)
    }

    /// Handle left mouse click using unified data providers
    fn handle_left_click(
        app: &mut App,
        mouse: MouseEvent,
        left_area: Rect,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Only handle clicks in the left panel (file/history list)
        if mouse.column < left_area.x || mouse.column >= left_area.x + left_area.width {
            return Ok(ModeAction::Stay);
        }

        let provider = create_data_provider(current_mode);

        // Calculate the actual clicked index considering scroll offset
        let visible_row = (mouse.row - left_area.y - 1) as usize; // Row relative to the visible area
        let scroll_offset = Self::get_scroll_offset(app, current_mode);
        let clicked_index = visible_row + scroll_offset;

        // Check bounds
        if clicked_index >= provider.get_total_count(app) {
            return Ok(ModeAction::Stay);
        }

        // Check for double-click
        let mouse_position = (mouse.column, mouse.row);
        let is_double_click = Self::is_double_click(app, mouse_position, clicked_index);

        // Update selection
        provider.set_selected_index(app, Some(clicked_index));
        Self::update_preview_if_needed(app, &*provider);

        // Update double-click state
        Self::update_double_click_state(app, mouse_position, clicked_index);

        // Handle double-click action
        if is_double_click {
            if let Some(item) = provider.get_selected_item(app) {
                match item {
                    DisplayItem::File(_) => {
                        if let Some(action) = provider.navigate_into_directory(app)? {
                            return Ok(action);
                        } else {
                            return Ok(ModeAction::Stay);
                        }
                    }
                    DisplayItem::HistoryPath(path) => {
                        let file_item = FileItem::from_path(&path);
                        return Ok(ModeAction::Exit(Some(file_item)));
                    }
                }
            }
        }

        Ok(ModeAction::Stay)
    }

    /// Check if current click is a double-click
    fn is_double_click(app: &App, mouse_position: (u16, u16), clicked_index: usize) -> bool {
        use std::time::Instant;
        const DOUBLE_CLICK_INTERVAL_MS: u64 = 150;

        let current_time = Instant::now();
        if let (Some(last_time), Some(last_pos), Some(last_idx)) = (
            app.state.double_click_state.last_click_time,
            app.state.double_click_state.last_click_position,
            app.state.double_click_state.last_clicked_index,
        ) {
            let elapsed = current_time.duration_since(last_time);
            elapsed.as_millis() <= DOUBLE_CLICK_INTERVAL_MS as u128
                && last_pos == mouse_position
                && last_idx == clicked_index
        } else {
            false
        }
    }

    /// Update double-click state
    fn update_double_click_state(app: &mut App, mouse_position: (u16, u16), clicked_index: usize) {
        use std::time::Instant;

        app.state.double_click_state.last_click_time = Some(Instant::now());
        app.state.double_click_state.last_click_position = Some(mouse_position);
        app.state.double_click_state.last_clicked_index = Some(clicked_index);
    }

    /// Get the current scroll offset for the given mode
    fn get_scroll_offset(app: &App, current_mode: &AppMode) -> usize {
        match current_mode {
            AppMode::History => app.state.history_state.offset(),
            _ => app.state.file_list_state.offset(), // Normal and Search modes use file_list_state
        }
    }
}
