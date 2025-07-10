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
        history::{HistoryHelpRenderer, HistoryListRenderer},
        shared::{PreviewRenderer, renderers::Renderer},
    },
    services::state::StateService,
};

/// Handler for History mode (navigate previous directories)
pub struct HistoryModeHandler {
    history_list_renderer: Box<dyn Renderer>,
    preview_renderer: Box<dyn Renderer>,
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
            history_list_renderer: Box::new(HistoryListRenderer::new()),
            preview_renderer: Box::new(PreviewRenderer::new()),
            help_renderer: Box::new(HistoryHelpRenderer::new()),
        }
    }
}

impl ModeHandler for HistoryModeHandler {
    fn render_left_panel(&self, f: &mut Frame, area: Rect, app: &App) {
        self.history_list_renderer.render(f, area, app);
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
                    "SEARCH - Type to search history, ESC to exit search".to_string(),
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
                    "FILTERED HISTORY - '{}' - {} matches (l/→ enter dir, / to search again, ESC to normal)",
                    app.state.search_input,
                    app.state.filtered_files.len()
                ),
                Style::default().fg(Color::Black).bg(Color::Green),
            )
        } else {
            (
                format!(
                    "HISTORY - {} entries (jk navigate, l/→ enter dir, / search, Enter select, ESC to normal)",
                    app.state.files.len()
                ),
                Style::default().fg(Color::Cyan),
            )
        };
        (info, app.state.search_input.clone(), style)
    }

    fn should_show_help(&self, app: &App) -> bool {
        // Show help if no selection or if searching with no results
        if app.state.is_searching {
            app.state.search_input.is_empty() || app.state.filtered_files.is_empty()
        } else {
            app.state.file_list_state.selected().is_none()
        }
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
