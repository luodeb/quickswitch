use anyhow::Result;
use crossterm::event::{KeyCode, MouseEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    app::App,
    modes::{ModeAction, ModeHandler},
    renderers::{Renderer, RendererType, create_renderer, should_show_help},
    services::state::StateService,
};

/// Handler for Search mode (interactive search)
pub struct SearchModeHandler {
    file_list_renderer: Box<dyn Renderer>,
    preview_renderer: Box<dyn Renderer>,
    help_renderer: Box<dyn Renderer>,
}

impl Default for SearchModeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchModeHandler {
    pub fn new() -> Self {
        Self {
            file_list_renderer: create_renderer(RendererType::FileList),
            preview_renderer: create_renderer(RendererType::Preview),
            help_renderer: create_renderer(RendererType::SearchHelp),
        }
    }
}

impl ModeHandler for SearchModeHandler {
    fn handle_key(&mut self, app: &mut App, key: KeyCode) -> Result<ModeAction> {
        match key {
            KeyCode::Esc => Ok(ModeAction::Switch(crate::models::AppMode::Normal)),
            KeyCode::Enter => {
                let selected_file = app.get_selected_file().cloned();
                Ok(ModeAction::Exit(selected_file))
            }
            // Only allow arrow keys for navigation in search mode (disable hjkl)
            KeyCode::Up => {
                crate::handlers::navigation::NavigationHelper::navigate_file_list_up(app);
                Ok(ModeAction::Stay)
            }
            KeyCode::Down => {
                crate::handlers::navigation::NavigationHelper::navigate_file_list_down(app);
                Ok(ModeAction::Stay)
            }
            KeyCode::Right => {
                crate::handlers::navigation::NavigationHelper::navigate_into_directory(app)?;
                Ok(ModeAction::Stay)
            }
            KeyCode::Left => {
                crate::handlers::navigation::NavigationHelper::navigate_to_parent(app)?;
                Ok(ModeAction::Stay)
            }
            KeyCode::Backspace => {
                StateService::remove_search_char(app);
                app.update_preview();
                Ok(ModeAction::Stay)
            }
            KeyCode::Char(c) => {
                StateService::add_search_char(app, c);
                app.update_preview();
                Ok(ModeAction::Stay)
            }
            _ => Ok(ModeAction::Stay),
        }
    }

    fn handle_mouse(&mut self, app: &mut App, mouse: MouseEvent, left_area: Rect, right_area: Rect) -> Result<ModeAction> {
        // Import CommonModeLogic for mouse handling
        use crate::modes::common::CommonModeLogic;
        
        // Handle position-aware mouse scroll navigation
        if CommonModeLogic::handle_position_aware_scroll_navigation(app, mouse, left_area, right_area)? {
            return Ok(ModeAction::Stay);
        }

        // Handle mouse click for file selection and navigation
        // Note: We need the area information to properly handle clicks
        // For now, we'll just handle scroll events
        Ok(ModeAction::Stay)
    }

    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.file_list_renderer.render(f, area, app);
    }

    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        if should_show_help(app, &crate::models::AppMode::Search) {
            self.help_renderer.render(f, area, app);
        } else {
            self.preview_renderer.render(f, area, app);
        }
    }

    fn get_search_box_config(&self, app: &App) -> (String, String, Style) {
        let info = if app.state.search_input.is_empty() {
            "SEARCH - Type to search, ESC to normal mode".to_string()
        } else {
            format!(
                "SEARCH - '{}' - {} matches (ESC to normal)",
                app.state.search_input,
                app.state.filtered_files.len()
            )
        };
        (
            info,
            app.state.search_input.clone(),
            Style::default().fg(Color::Black).bg(Color::Yellow),
        )
    }

    fn on_enter(&mut self, _app: &mut App) -> Result<()> {
        // Search mode starts fresh, no special initialization needed
        Ok(())
    }

    fn on_exit(&mut self, app: &mut App) -> Result<()> {
        // Clear search when exiting search mode
        StateService::clear_search(app);
        app.update_preview();
        Ok(())
    }
}
