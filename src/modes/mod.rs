use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Rect, style::Style};

use crate::app::App;

pub mod common;
pub mod history;
pub mod normal;
pub mod search;

/// Core trait that defines the interface for all application modes
pub trait ModeHandler {
    /// Handle keyboard input for this mode
    fn handle_key(&mut self, app: &mut App, key: KeyCode) -> Result<bool>;

    /// Render the left panel (file list or history list)
    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App);

    /// Render the right panel (preview or help)
    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App);

    /// Get search box configuration (title, content, style)
    fn get_search_box_config(&self, app: &App) -> (String, String, Style);

    /// Called when entering this mode
    fn on_enter(&mut self, app: &mut App) -> Result<()>;

    /// Called when exiting this mode
    fn on_exit(&mut self, app: &mut App) -> Result<()>;
}

/// Factory function to create mode handlers
pub fn create_mode_handler(mode: &crate::models::AppMode) -> Box<dyn ModeHandler> {
    match mode {
        crate::models::AppMode::Normal => Box::new(normal::NormalModeHandler::new()),
        crate::models::AppMode::Search => Box::new(search::SearchModeHandler::new()),
        crate::models::AppMode::History => Box::new(history::HistoryModeHandler::new()),
    }
}

/// Mode manager that coordinates between different modes
pub struct ModeManager {
    current_handler: Box<dyn ModeHandler>,
}

impl ModeManager {
    pub fn new(initial_mode: &crate::models::AppMode) -> Self {
        Self {
            current_handler: create_mode_handler(initial_mode),
        }
    }

    pub fn switch_mode(&mut self, app: &mut App, new_mode: &crate::models::AppMode) -> Result<()> {
        self.current_handler.on_exit(app)?;
        self.current_handler = create_mode_handler(new_mode);
        self.current_handler.on_enter(app)?;
        Ok(())
    }

    pub fn handle_key(&mut self, app: &mut App, key: KeyCode) -> Result<bool> {
        self.current_handler.handle_key(app, key)
    }

    pub fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.current_handler.render_left_panel(f, area, app);
    }

    pub fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.current_handler.render_right_panel(f, area, app);
    }

    pub fn get_search_box_config(&self, app: &App) -> (String, String, Style) {
        self.current_handler.get_search_box_config(app)
    }
}
