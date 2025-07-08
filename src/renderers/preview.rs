use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for preview panel showing file/directory content
#[derive(Default)]
pub struct PreviewRenderer;

impl PreviewRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for PreviewRenderer {
    fn render(&self, f: &mut Frame, area: Rect, app: &App) {
        // Calculate the visible content based on scroll offset
        let total_lines = app.state.preview_content.len();
        let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
        let scroll_offset = app.state.preview_scroll_offset;

        // Determine the range of lines to display
        let start_line = scroll_offset;
        let end_line = (start_line + visible_height).min(total_lines);

        // Get the visible content slice
        let visible_content: Vec<_> = if start_line < total_lines {
            app.state.preview_content[start_line..end_line]
                .iter()
                .map(|line| ListItem::new(line.clone()))
                .collect()
        } else {
            vec![]
        };

        let preview_list = List::new(visible_content).block(
            Block::default()
                .borders(Borders::ALL)
                .title(&*app.state.preview_title),
        );

        f.render_widget(preview_list, area);
    }
}
