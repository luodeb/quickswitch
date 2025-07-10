use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};

use crate::{
    AppState,
    modes::ModeAction,
    services::{DataProvider, PreviewManager, create_data_provider},
    utils::{AppMode, DisplayItem, FileItem},
};

/// Unified input dispatcher for handling all user interactions
/// This centralizes key and mouse event handling, reducing duplication across modes
pub struct InputDispatcher;

impl InputDispatcher {
    /// Handle keyboard input uniformly across all modes
    pub fn handle_key_event(
        app_state: &mut AppState,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Handle exit keys first (highest priority)
        if let Some(action) = Self::handle_exit_keys(app_state, key, current_mode) {
            return Ok(action);
        }

        // Handle mode switch keys
        if let Some(action) = Self::handle_mode_switch_keys(app_state, key, current_mode) {
            return Ok(action);
        }

        // Handle navigation keys (unified for all modes)
        if let Some(action) = Self::handle_navigation_keys(app_state, key, current_mode)? {
            return Ok(action);
        }

        // Handle mode-specific keys
        Self::handle_mode_specific_keys(app_state, key, current_mode)
    }

    /// Handle mouse input uniformly across all modes
    pub fn handle_mouse_event(
        app_state: &mut AppState,
        mouse: MouseEvent,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        match mouse.kind {
            MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                Self::handle_scroll_navigation(app_state, mouse, current_mode)
            }
            MouseEventKind::Up(crossterm::event::MouseButton::Left) => {
                Self::handle_left_click(app_state, mouse, current_mode)
            }
            MouseEventKind::Up(crossterm::event::MouseButton::Right) => {
                Self::handle_right_click(app_state, mouse, current_mode)
            },
            _ => Ok(ModeAction::Stay),
        }
    }

    /// Handle exit keys (Esc, Enter) - unified across all modes
    fn handle_exit_keys(
        app_state: &mut AppState,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Option<ModeAction> {
        match key {
            KeyCode::Esc => {
                // If searching, exit search mode but keep search input and results
                if app_state.is_searching {
                    app_state.is_searching = false;
                    // Don't clear search_input - keep the search results visible
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
                if let Some(item) = provider.get_selected_item(app_state) {
                    let _ = provider.navigate_to_selected(app_state);
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
        app_state: &mut AppState,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Option<ModeAction> {
        match key {
            KeyCode::Char('/') => {
                // Enable search functionality in normal and history modes
                if matches!(current_mode, AppMode::Normal | AppMode::History)
                    && !app_state.is_searching
                {
                    app_state.is_searching = true;
                    Some(ModeAction::Stay)
                } else {
                    None
                }
            }
            KeyCode::Char('v') if !app_state.is_searching => {
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
        app_state: &mut AppState,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Result<Option<ModeAction>> {
        let provider = create_data_provider(current_mode);

        match key {
            KeyCode::Up => {
                provider.navigate_up(app_state);
                Self::update_preview_if_needed(app_state, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Down => {
                provider.navigate_down(app_state);
                Self::update_preview_if_needed(app_state, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Right => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_into_directory(app_state)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::Left => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_to_parent(app_state)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            // hjkl keys only work when not searching
            KeyCode::Char('k') if !app_state.is_searching => {
                provider.navigate_up(app_state);
                Self::update_preview_if_needed(app_state, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Char('j') if !app_state.is_searching => {
                provider.navigate_down(app_state);
                Self::update_preview_if_needed(app_state, &*provider);
                Ok(Some(ModeAction::Stay))
            }
            KeyCode::Char('l') if !app_state.is_searching => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_into_directory(app_state)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::Char('h') if !app_state.is_searching => {
                // Use provider's navigation method
                if let Some(action) = provider.navigate_to_parent(app_state)? {
                    Ok(Some(action))
                } else {
                    Ok(Some(ModeAction::Stay))
                }
            }
            KeyCode::PageUp | KeyCode::PageDown => {
                Self::handle_preview_navigation(app_state, key);
                Ok(Some(ModeAction::Stay))
            }
            _ => Ok(None),
        }
    }

    /// Handle mode-specific keys that don't fit into common patterns
    fn handle_mode_specific_keys(
        app_state: &mut AppState,
        key: KeyCode,
        _current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Handle search input when in search mode
        if app_state.is_searching {
            Self::handle_search_keys(app_state, key)
        } else {
            Ok(ModeAction::Stay)
        }
    }

    /// Handle search mode specific keys
    fn handle_search_keys(app_state: &mut AppState, key: KeyCode) -> Result<ModeAction> {
        match key {
            KeyCode::Char(c) => {
                app_state.search_input.push(c);
                app_state.apply_search_filter();
                Ok(ModeAction::Stay)
            }
            KeyCode::Backspace => {
                app_state.search_input.pop();
                app_state.apply_search_filter();
                Ok(ModeAction::Stay)
            }
            _ => Ok(ModeAction::Stay),
        }
    }

    /// Handle preview navigation (Page Up/Down)
    fn handle_preview_navigation(app_state: &mut AppState, key: KeyCode) {
        // Use the actual right panel content height from layout manager
        let visible_height = app_state.layout.get_right_content_height();
        match key {
            KeyCode::PageUp => {
                PreviewManager::scroll_preview_page_up(app_state, visible_height);
            }
            KeyCode::PageDown => {
                PreviewManager::scroll_preview_page_down(app_state, visible_height);
            }
            _ => {}
        }
    }

    /// Update preview if the data provider supports it
    fn update_preview_if_needed(app_state: &mut AppState, provider: &dyn DataProvider) {
        if let Some(item) = provider.get_selected_item(app_state) {
            PreviewManager::update_preview_for_item(app_state, &item);
        }
    }

    /// Handle scroll navigation using unified data providers
    fn handle_scroll_navigation(
        app_state: &mut AppState,
        mouse: MouseEvent,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        let is_scroll_up = matches!(mouse.kind, MouseEventKind::ScrollUp);

        // Check if mouse is in left area (file/history list) or right area (preview)
        if app_state.is_point_in_left_panel(mouse.column, mouse.row) {
            // Mouse is in left panel - scroll list using unified provider
            let provider = create_data_provider(current_mode);
            if is_scroll_up {
                provider.navigate_up(app_state);
            } else {
                provider.navigate_down(app_state);
            }
            Self::update_preview_if_needed(app_state, &*provider);
        } else if app_state.is_point_in_right_panel(mouse.column, mouse.row) {
            // Mouse is in right panel - scroll preview content
            if is_scroll_up {
                PreviewManager::scroll_preview_up(app_state);
            } else {
                PreviewManager::scroll_preview_down(app_state);
            }
        }

        Ok(ModeAction::Stay)
    }

    fn handle_right_click(
        app_state: &mut AppState,
        _mouse: MouseEvent,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        let provider = create_data_provider(current_mode);
        // Use provider's navigation method
        if let Some(action) = provider.navigate_to_parent(app_state)? {
            Ok(action)
        } else {
            Ok(ModeAction::Stay)
        }
    }

    /// Handle left mouse click using unified data providers
    fn handle_left_click(
        app_state: &mut AppState,
        mouse: MouseEvent,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        // Only handle clicks in the left panel (file/history list)
        if !app_state.is_point_in_left_panel(mouse.column, mouse.row) {
            return Ok(ModeAction::Stay);
        }

        let provider = create_data_provider(current_mode);
        let left_area = app_state.layout.get_left_area();

        // Calculate the actual clicked index considering scroll offset
        let visible_row = (mouse.row - left_area.y - 1) as usize; // Row relative to the visible area
        let scroll_offset = Self::get_scroll_offset(app_state, current_mode);
        let clicked_index = visible_row + scroll_offset;

        // Check bounds
        if clicked_index >= provider.get_total_count(app_state) {
            return Ok(ModeAction::Stay);
        }

        // Check for double-click
        let mouse_position = (mouse.column, mouse.row);
        let is_double_click = Self::is_double_click(app_state, mouse_position, clicked_index);

        // Update selection
        provider.set_selected_index(app_state, Some(clicked_index));
        Self::update_preview_if_needed(app_state, &*provider);

        // Update double-click state
        Self::update_double_click_state(app_state, mouse_position, clicked_index);

        // Handle double-click action
        if is_double_click {
            if let Some(item) = provider.get_selected_item(app_state) {
                match item {
                    DisplayItem::File(_) => {
                        if let Some(action) = provider.navigate_into_directory(app_state)? {
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
    fn is_double_click(
        app_state: &mut AppState,
        mouse_position: (u16, u16),
        clicked_index: usize,
    ) -> bool {
        use std::time::Instant;
        const DOUBLE_CLICK_INTERVAL_MS: u64 = 150;

        let current_time = Instant::now();
        if let (Some(last_time), Some(last_pos), Some(last_idx)) = (
            app_state.double_click_state.last_click_time,
            app_state.double_click_state.last_click_position,
            app_state.double_click_state.last_clicked_index,
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
    fn update_double_click_state(
        app_state: &mut AppState,
        mouse_position: (u16, u16),
        clicked_index: usize,
    ) {
        use std::time::Instant;

        app_state.double_click_state.last_click_time = Some(Instant::now());
        app_state.double_click_state.last_click_position = Some(mouse_position);
        app_state.double_click_state.last_clicked_index = Some(clicked_index);
    }

    /// Get the current scroll offset for the given mode
    fn get_scroll_offset(app_state: &mut AppState, _current_mode: &AppMode) -> usize {
        // All modes now use the unified file_list_state
        app_state.file_list_state.offset()
    }
}
