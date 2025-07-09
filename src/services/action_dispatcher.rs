use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::layout::Rect;

use crate::{app::App, core::InputDispatcher, models::AppMode, modes::ModeAction};

/// Legacy action dispatcher - now delegates to InputDispatcher
/// This is kept for backward compatibility during the transition
pub struct ActionDispatcher;

impl ActionDispatcher {
    /// Handle keyboard input - delegates to InputDispatcher
    pub fn handle_key_event(
        app: &mut App,
        key: KeyCode,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        InputDispatcher::handle_key_event(app, key, current_mode)
    }

    /// Handle mouse input - delegates to InputDispatcher
    pub fn handle_mouse_event(
        app: &mut App,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
        current_mode: &AppMode,
    ) -> Result<ModeAction> {
        InputDispatcher::handle_mouse_event(app, mouse, left_area, right_area, current_mode)
    }
}
