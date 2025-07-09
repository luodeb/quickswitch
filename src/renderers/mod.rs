use ratatui::{Frame, layout::Rect};

use crate::app::App;

pub mod file_list;
pub mod help;
pub mod history_list;
pub mod preview;

/// Core trait for UI rendering components
pub trait Renderer {
    /// Render the component in the given area
    fn render(&self, f: &mut Frame, area: Rect, app: &App);
}

/// Types of renderers available
pub enum RendererType {
    FileList,
    HistoryList,
    Preview,
    NormalHelp,
    SearchHelp,
    HistoryHelp,
}

/// Factory function to create renderers
pub fn create_renderer(renderer_type: RendererType) -> Box<dyn Renderer> {
    match renderer_type {
        RendererType::FileList => Box::new(file_list::FileListRenderer::new()),
        RendererType::HistoryList => Box::new(history_list::HistoryListRenderer::new()),
        RendererType::Preview => Box::new(preview::PreviewRenderer::new()),
        RendererType::NormalHelp => Box::new(help::NormalHelpRenderer::new()),
        RendererType::SearchHelp => Box::new(help::SearchHelpRenderer::new()),
        RendererType::HistoryHelp => Box::new(help::HistoryHelpRenderer::new()),
    }
}

/// Helper function to determine if help should be shown
pub fn should_show_help(app: &App, mode: &crate::models::AppMode) -> bool {
    match mode {
        crate::models::AppMode::Normal => {
            // Show help if no selection or if searching with no results
            if app.state.is_searching {
                app.state.search_input.is_empty() || app.state.filtered_files.is_empty()
            } else {
                app.state.file_list_state.selected().is_none() || app.state.filtered_files.is_empty()
            }
        }
        crate::models::AppMode::History => {
            app.state.history_state.selected().is_none() || app.state.history.is_empty()
        }
    }
}
