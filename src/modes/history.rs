use anyhow::Result;
use crossterm::event::{KeyCode, ModifierKeyCode, MouseEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    app::App,
    modes::{ModeAction, ModeHandler, common::CommonModeLogic},
    renderers::{Renderer, RendererType, create_renderer},
    services::state::StateService,
};

/// Handler for History mode (navigate previous directories)
pub struct HistoryModeHandler {
    history_list_renderer: Box<dyn Renderer>,
    help_renderer: Box<dyn Renderer>,
}

impl Default for HistoryModeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryModeHandler {
    pub fn new() -> Self {
        Self {
            history_list_renderer: create_renderer(RendererType::HistoryList),
            help_renderer: create_renderer(RendererType::HistoryHelp),
        }
    }
}

impl ModeHandler for HistoryModeHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyCode) -> Result<ModeAction> {
        match key {
            KeyCode::Esc => Ok(ModeAction::Switch(crate::models::AppMode::Normal)),
            KeyCode::Enter => {
                // Check if it's Ctrl+Enter for special behavior
                if key.is_modifier(ModifierKeyCode::LeftControl) {
                    // Ctrl+Enter: Navigate to directory and stay in history mode
                    if let Some(selected) = app.state.history_state.selected() {
                        if let Some(selected_path) =
                            StateService::move_history_to_front(app, selected)
                        {
                            app.save_history().unwrap_or(());
                            app.change_directory(selected_path)?;
                            return Ok(ModeAction::Switch(crate::models::AppMode::Normal));
                        }
                    }
                } else {
                    // Regular Enter: Exit with selected directory
                    if let Some(selected) = app.state.history_state.selected() {
                        if let Some(path) = app.state.history.get(selected) {
                            let file_item = crate::models::FileItem {
                                name: path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .into_owned(),
                                path: path.to_path_buf(),
                                is_dir: path.is_dir(),
                            };
                            return Ok(ModeAction::Exit(Some(file_item)));
                        }
                    }
                }
                Ok(ModeAction::Stay)
            }
            _ => {
                // Handle history navigation
                if CommonModeLogic::handle_history_navigation(app, key)? {
                    return Ok(ModeAction::Stay);
                }
                // Disable other navigation in history mode
                Ok(ModeAction::Stay)
            }
        }
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent) -> Result<ModeAction> {
        // Handle mouse scroll for history navigation
        use crossterm::event::MouseEventKind;
        use crate::handlers::navigation::NavigationHelper;
        
        match mouse.kind {
            MouseEventKind::ScrollUp => {
                NavigationHelper::navigate_history_up(app);
                Ok(ModeAction::Stay)
            }
            MouseEventKind::ScrollDown => {
                NavigationHelper::navigate_history_down(app);
                Ok(ModeAction::Stay)
            }
            _ => Ok(ModeAction::Stay),
        }
    }

    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.history_list_renderer.render(f, area, app);
    }

    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        // Always show help in history mode
        self.help_renderer.render(f, area, app);
    }

    fn get_search_box_config(&self, app: &App) -> (String, String, Style) {
        let info = format!(
            "HISTORY - {} entries (jk navigate, Enter select, ESC to normal)",
            app.state.history.len()
        );
        (info, String::new(), Style::default().fg(Color::Cyan))
    }

    fn on_enter(&mut self, app: &mut App) -> Result<()> {
        // Initialize history mode selection
        StateService::initialize_history_mode(app);
        Ok(())
    }

    fn on_exit(&mut self, _app: &mut App) -> Result<()> {
        // No special cleanup needed for history mode
        Ok(())
    }
}
