use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    app::App,
    modes::{
        ModeHandler,
        normal::{FileListRenderer, NormalHelpRenderer},
        shared::{PreviewRenderer, renderers::Renderer},
    },
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
            file_list_renderer: Box::new(FileListRenderer::new()),
            preview_renderer: Box::new(PreviewRenderer::new()),
            help_renderer: Box::new(NormalHelpRenderer::new()),
        }
    }
}

impl ModeHandler for NormalModeHandler {
    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.file_list_renderer.render(f, area, app);
    }

    fn render_right_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        if self.should_show_help(app) {
            self.help_renderer.render(f, area, app);
        } else {
            self.preview_renderer.render(f, area, app);
        }
    }

    fn get_search_box_config(&self, app: &App) -> (String, String, Style) {
        let (info, style) = if app.state.is_searching {
            if app.state.search_input.is_empty() {
                (
                    "SEARCH - Type to search, ESC to exit search".to_string(),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                )
            } else {
                (
                    format!(
                        "SEARCH - '{}' - {} matches (ESC to exit)",
                        app.state.search_input,
                        app.state.filtered_files.len()
                    ),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                )
            }
        } else if !app.state.search_input.is_empty() {
            // Show search results even when not actively searching
            (
                format!(
                    "FILTERED - '{}' - {} matches (/ to search again)",
                    app.state.search_input,
                    app.state.filtered_files.len()
                ),
                Style::default().fg(Color::Black).bg(Color::Green),
            )
        } else {
            (
                "NORMAL - hjkl navigate, / search, V history, Enter exit".to_string(),
                Style::default().fg(Color::Yellow),
            )
        };
        (info, app.state.search_input.clone(), style)
    }

    fn should_show_help(&self, app: &App) -> bool {
        // Show help if no selection or if searching with no results
        if app.state.is_searching {
            app.state.search_input.is_empty() || app.state.filtered_files.is_empty()
        } else {
            app.state.file_list_state.selected().is_none() || app.state.filtered_files.is_empty()
        }
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
