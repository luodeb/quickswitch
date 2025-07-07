use anyhow::Result;
use crossterm::event::KeyCode;

use crate::app::App;

pub mod navigation;

/// Trait for handling keyboard events
pub trait KeyEventHandler {
    /// Handle navigation keys (up, down, left, right, j, k, h, l)
    fn handle_navigation(&self, app: &mut App, key: KeyCode) -> Result<bool>;

    /// Handle mode switching keys (/, v, Esc)
    fn handle_mode_switch(&self, app: &mut App, key: KeyCode) -> Result<bool>;

    /// Handle special keys specific to each mode
    fn handle_special_keys(&self, app: &mut App, key: KeyCode) -> Result<bool>;
}

/// Common navigation actions
pub enum NavigationAction {
    Up,
    Down,
    Left,
    Right,
    Select,
    Cancel,
}

/// Convert key codes to navigation actions
pub fn key_to_navigation_action(key: KeyCode) -> Option<NavigationAction> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => Some(NavigationAction::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(NavigationAction::Down),
        KeyCode::Left | KeyCode::Char('h') => Some(NavigationAction::Left),
        KeyCode::Right | KeyCode::Char('l') => Some(NavigationAction::Right),
        KeyCode::Enter => Some(NavigationAction::Select),
        KeyCode::Esc => Some(NavigationAction::Cancel),
        _ => None,
    }
}
