use anyhow::Result;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    AppState,
    modes::{
        ModeHandler, Renderer,
        history::{HistoryHelpRenderer, HistoryListRenderer},
        preview::PreviewRenderer,
    },
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
    fn render_left_panel(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.history_list_renderer.render(f, area, state);
    }

    fn render_right_panel(&self, f: &mut Frame, area: Rect, state: &AppState) {
        if self.should_show_help(state) {
            self.help_renderer.render(f, area, state);
        } else {
            self.preview_renderer.render(f, area, state);
        }
    }

    fn get_search_box_config(&self, state: &AppState) -> (String, String, Style) {
        let (info, style) = if state.is_searching {
            if state.search_input.is_empty() {
                (
                    "SEARCH - Type to search history, ESC to exit search".to_string(),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                )
            } else {
                (
                    format!(
                        "SEARCH - '{}' - {} matches (ESC to exit)",
                        state.search_input,
                        state.filtered_files.len()
                    ),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                )
            }
        } else if !state.search_input.is_empty() {
            // Show search results even when not actively searching
            (
                format!(
                    "FILTERED HISTORY - '{}' - {} matches (l/→ enter dir, / to search again, ESC to normal)",
                    state.search_input,
                    state.filtered_files.len()
                ),
                Style::default().fg(Color::Black).bg(Color::Green),
            )
        } else {
            (
                format!(
                    "HISTORY - {} entries (jk navigate, l/→ enter dir, / search, Enter select, ESC to normal)",
                    state.files.len()
                ),
                Style::default().fg(Color::Cyan),
            )
        };
        (info, state.search_input.clone(), style)
    }

    fn should_show_help(&self, state: &AppState) -> bool {
        // Show help if no selection or if searching with no results
        if state.is_searching {
            state.search_input.is_empty() || state.filtered_files.is_empty()
        } else {
            state.file_list_state.selected().is_none()
        }
    }

    fn on_enter(&mut self, state: &mut AppState) -> Result<()> {
        // Initialize history mode selection
        state.file_list_state.select(None);
        Ok(())
    }
}
