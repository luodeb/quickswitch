use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{Frame, layout::Rect, style::Style};

use crate::{FileItem, app::App, events::handle_exit, models::AppMode, services::ActionDispatcher};

pub mod history;
pub mod normal;
pub mod shared;

/// Represents a mode switch request
#[derive(Debug, Clone, PartialEq)]
pub enum ModeAction {
    Stay,
    Switch(AppMode),
    Exit(Option<FileItem>),
}

/// Simplified trait that defines the interface for all application modes
/// Each mode focuses on its core rendering and initialization logic
/// All input handling is now unified through InputDispatcher
pub trait ModeHandler {
    /// Render the left panel (file list or history list)
    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App);

    /// Render the right panel (preview or help)
    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App);

    /// Get search box configuration (title, content, style)
    fn get_search_box_config(&self, app: &App) -> (String, String, Style);

    /// Determine if help should be shown instead of preview
    fn should_show_help(&self, app: &App) -> bool;

    /// Called when entering this mode
    fn on_enter(&mut self, app: &mut App) -> Result<()>;

    /// Called when exiting this mode
    fn on_exit(&mut self, app: &mut App) -> Result<()>;
}

/// Factory function to create mode handlers
pub fn create_mode_handler(mode: &AppMode) -> Box<dyn ModeHandler> {
    match mode {
        AppMode::Normal => Box::new(normal::NormalModeHandler::new()),
        AppMode::History => Box::new(history::HistoryModeHandler::new()),
    }
}

/// Mode manager that coordinates between different modes
pub struct ModeManager {
    current_handler: Box<dyn ModeHandler>,
    current_mode: AppMode,
}

/// App controller that coordinates between App and ModeManager
pub struct AppController {
    app: App,
    mode_manager: ModeManager,
}

impl ModeManager {
    pub fn new(initial_mode: &AppMode) -> Self {
        Self {
            current_handler: create_mode_handler(initial_mode),
            current_mode: initial_mode.clone(),
        }
    }

    pub fn switch_mode(&mut self, app: &mut App, new_mode: &AppMode) -> Result<()> {
        self.current_handler.on_exit(app)?;

        // Clear search when switching modes
        app.state.search_input.clear();
        app.state.is_searching = false;

        // Load appropriate data for the new mode using data provider
        let data_provider = crate::services::create_data_provider(new_mode);
        data_provider.load_data(app)?;

        self.current_handler = create_mode_handler(new_mode);
        self.current_mode = new_mode.clone();
        self.current_handler.on_enter(app)?;
        Ok(())
    }

    pub fn handle_key(&mut self, app: &mut App, key: KeyCode) -> Result<ModeAction> {
        ActionDispatcher::handle_key_event(app, key, &self.current_mode)
    }

    pub fn handle_mouse(
        &mut self,
        app: &mut App,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
    ) -> Result<ModeAction> {
        ActionDispatcher::handle_mouse_event(app, mouse, left_area, right_area, &self.current_mode)
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

    pub fn get_current_mode(&self) -> &AppMode {
        &self.current_mode
    }

    pub fn is_mode(&self, mode: &AppMode) -> bool {
        self.current_mode == *mode
    }
}

impl AppController {
    pub fn new(initial_mode: AppMode) -> Result<Self> {
        let mut app = App::new()?;

        // Load initial data using data provider
        let data_provider = crate::services::create_data_provider(&initial_mode);
        data_provider.load_data(&mut app)?;

        // Clear preview
        crate::services::PreviewManager::clear_preview(&mut app);

        let mode_manager = ModeManager::new(&initial_mode);
        Ok(Self { app, mode_manager })
    }

    pub fn switch_mode(&mut self, new_mode: AppMode) -> Result<()> {
        self.mode_manager.switch_mode(&mut self.app, &new_mode)
    }

    pub fn handle_key(&mut self, key: KeyCode) -> Result<bool> {
        let action = self.mode_manager.handle_key(&mut self.app, key)?;
        match action {
            ModeAction::Stay => Ok(true),
            ModeAction::Switch(new_mode) => {
                self.switch_mode(new_mode)?;
                Ok(true)
            }
            ModeAction::Exit(file_item) => {
                handle_exit(self, file_item.as_ref())?;
                Ok(false) // This should never be reached due to process::exit in handle_exit
            }
        }
    }

    pub fn handle_mouse(
        &mut self,
        mouse: MouseEvent,
        left_area: Rect,
        right_area: Rect,
    ) -> Result<bool> {
        let action = self
            .mode_manager
            .handle_mouse(&mut self.app, mouse, left_area, right_area)?;
        match action {
            ModeAction::Stay => Ok(true),
            ModeAction::Switch(new_mode) => {
                self.switch_mode(new_mode)?;
                Ok(true)
            }
            ModeAction::Exit(file_item) => {
                handle_exit(self, file_item.as_ref())?;
                Ok(false) // This should never be reached due to process::exit in handle_exit
            }
        }
    }

    pub fn render_left_panel(&self, f: &mut Frame, area: Rect) {
        self.mode_manager.render_left_panel(f, area, &self.app);
    }

    pub fn render_right_panel(&self, f: &mut Frame, area: Rect) {
        self.mode_manager.render_right_panel(f, area, &self.app);
    }

    pub fn get_search_box_config(&self) -> (String, String, Style) {
        self.mode_manager.get_search_box_config(&self.app)
    }

    pub fn get_current_mode(&self) -> &AppMode {
        self.mode_manager.get_current_mode()
    }

    pub fn is_mode(&self, mode: &AppMode) -> bool {
        self.mode_manager.is_mode(mode)
    }

    pub fn get_app(&self) -> &App {
        &self.app
    }

    pub fn get_app_mut(&mut self) -> &mut App {
        &mut self.app
    }
}
