use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

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
            Line::from("/          - Enter search mode"),
            Line::from("v          - Enter history mode"),
            Line::from("Enter      - Select and exit"),
            Line::from("Esc        - Exit application"),
            Line::from(""),
            Line::from("Mouse:"),
            Line::from("Click      - Select item"),
            Line::from("Double-click - Select and exit"),
            Line::from("Scroll     - Navigate list"),
            Line::from(""),
            Line::from("Preview:"),
            Line::from("PageUp/Down - Scroll preview"),
        ];

        let help_items: Vec<ListItem> = help_content
            .into_iter()
            .map(ListItem::new)
            .collect();

        let help_widget = List::new(help_items)
            .block(Block::default().title("Help").borders(Borders::ALL));

        f.render_widget(help_widget, area);
    }
}
