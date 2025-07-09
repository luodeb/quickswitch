use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, modes::shared::renderers::Renderer};

/// Renderer for Normal mode help
#[derive(Default)]
pub struct NormalHelpRenderer;

impl NormalHelpRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for NormalHelpRenderer {
    fn render(&self, f: &mut Frame, area: Rect, _app: &App) {
        let help_content = vec![
            Line::from("Normal Mode Navigation:"),
            Line::from(""),
            Line::from("h/←        - Go to parent directory"),
            Line::from("j/↓        - Move down"),
            Line::from("k/↑        - Move up"),
            Line::from("l/→        - Enter directory"),
            Line::from(""),
            Line::from("/          - Search files"),
            Line::from("ESC        - Exit search (when searching)"),
            Line::from("V          - Enter history mode"),
            Line::from("Enter      - Select and exit"),
            Line::from("ESC        - Quit application (when not searching)"),
        ];

        let help_items: Vec<ListItem> = help_content.into_iter().map(ListItem::new).collect();

        let help_widget = List::new(help_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Normal Mode"),
        );

        f.render_widget(help_widget, area);
    }
}
