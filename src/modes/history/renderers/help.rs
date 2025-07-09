use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for History mode help
#[derive(Default)]
pub struct HistoryHelpRenderer;

impl HistoryHelpRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for HistoryHelpRenderer {
    fn render(&self, f: &mut Frame, area: Rect, _app: &App) {
        let help_content = vec![
            Line::from("History Mode:"),
            Line::from(""),
            Line::from("Navigate through previously visited directories"),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("j/↓        - Move down in history"),
            Line::from("k/↑        - Move up in history"),
            Line::from("h/l/←/→    - Return to normal mode"),
            Line::from(""),
            Line::from("Actions:"),
            Line::from("/          - Enter search mode"),
            Line::from("Enter      - Select and exit"),
            Line::from("Esc        - Return to normal mode"),
            Line::from(""),
            Line::from("Mouse:"),
            Line::from("Click      - Select directory"),
            Line::from("Double-click - Select and exit"),
            Line::from("Scroll     - Navigate history"),
            Line::from(""),
            Line::from("Note:"),
            Line::from("History shows directories you've visited"),
            Line::from("Only existing directories are shown"),
        ];

        let help_items: Vec<ListItem> = help_content
            .into_iter()
            .map(ListItem::new)
            .collect();

        let help_widget = List::new(help_items)
            .block(Block::default().title("History Help").borders(Borders::ALL));

        f.render_widget(help_widget, area);
    }
}
