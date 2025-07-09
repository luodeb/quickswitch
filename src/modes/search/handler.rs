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
    services::{state::StateService, PreviewManager},
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
        PreviewManager::update_preview_from_selection(app);
        Ok(())
    }


}
