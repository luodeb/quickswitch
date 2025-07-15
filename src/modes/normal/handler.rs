use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

use crate::{
    app_state::AppState,
    modes::{
        ModeHandler, Renderer,
        normal::{FileListRenderer, NormalHelpRenderer},
        preview::PreviewRenderer,
    },
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
    fn render_left_panel(&self, f: &mut Frame, area: Rect, state: &AppState) {
        self.file_list_renderer.render(f, area, state);
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
                    "SEARCH - Type to search, ESC to exit search".to_string(),
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
                    "FILTERED - '{}' - {} matches (/f to search again)",
                    state.search_input,
                    state.filtered_files.len()
                ),
                Style::default().fg(Color::Black).bg(Color::Green),
            )
        } else {
            (
                "NORMAL - hjkl navigate, b/f half page, /f search, v history, Enter exit"
                    .to_string(),
                Style::default().fg(Color::Yellow),
            )
        };
        (info, state.search_input.clone(), style)
    }

    fn should_show_help(&self, state: &AppState) -> bool {
        // Show help if no selection or if searching with no results
        if state.is_searching {
            state.search_input.is_empty() || state.filtered_files.is_empty()
        } else {
            state.file_list_state.selected().is_none() || state.filtered_files.is_empty()
        }
    }
}
