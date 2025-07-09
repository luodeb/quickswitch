use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    app::App,
    modes::ModeHandler,
    renderers::{Renderer, RendererType, create_renderer, should_show_help},
    services::state::StateService,
};

/// Handler for Normal mode (default navigation mode)
pub struct NormalModeHandler {
    file_list_renderer: Box<dyn Renderer>,
    preview_renderer: Box<dyn Renderer>,
    help_renderer: Box<dyn Renderer>,
}

impl Default for NormalModeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl NormalModeHandler {
    pub fn new() -> Self {
        Self {
            file_list_renderer: create_renderer(RendererType::FileList),
            preview_renderer: create_renderer(RendererType::Preview),
            help_renderer: create_renderer(RendererType::NormalHelp),
        }
    }
}

impl ModeHandler for NormalModeHandler {
    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.file_list_renderer.render(f, area, app);
    }

    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        if should_show_help(app, &crate::models::AppMode::Normal) {
            self.help_renderer.render(f, area, app);
        } else {
            self.preview_renderer.render(f, area, app);
        }
    }

    fn get_search_box_config(&self, app: &App) -> (String, String, Style) {
        let info = if app.state.search_input.is_empty() {
            "NORMAL - hjkl navigate, / search, V history, Enter exit".to_string()
        } else {
            format!(
                "NORMAL - Search: '{}' - {} matches",
                app.state.search_input,
                app.state.filtered_files.len()
            )
        };
        (
            info,
            app.state.search_input.clone(),
            Style::default().fg(Color::Yellow),
        )
    }

    fn on_enter(&mut self, _app: &mut App) -> Result<()> {
        // Normal mode is the default, no special initialization needed
        Ok(())
    }

    fn on_exit(&mut self, app: &mut App) -> Result<()> {
        // Save current position before leaving normal mode
        StateService::save_current_position(app);
        Ok(())
    }


}
