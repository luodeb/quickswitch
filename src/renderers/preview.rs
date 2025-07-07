use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for preview panel showing file/directory content
pub struct PreviewRenderer;

impl PreviewRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for PreviewRenderer {
    fn render(&self, f: &mut Frame, area: Rect, app: &App) {
        let preview_list = List::new(
            app.state
                .preview_content
                .iter()
                .map(|line| ListItem::new(line.clone()))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(&*app.state.preview_title),
        );

        f.render_widget(preview_list, area);
    }
}
