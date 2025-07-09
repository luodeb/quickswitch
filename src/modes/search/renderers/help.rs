use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for Search mode help
pub struct SearchHelpRenderer;

impl Default for SearchHelpRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchHelpRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for SearchHelpRenderer {
    fn render(&self, f: &mut Frame, area: Rect, _app: &App) {
        let help_content = vec![
            Line::from("Search Mode:"),
            Line::from(""),
            Line::from("Type to search files and directories"),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("j/↓        - Move down in results"),
            Line::from("k/↑        - Move up in results"),
            Line::from("h/←        - Go to parent directory"),
            Line::from("l/→        - Enter directory"),
            Line::from(""),
            Line::from("Text Input:"),
            Line::from("Backspace  - Delete character"),
            Line::from("Any char   - Add to search"),
            Line::from(""),
            Line::from("Mode Switch:"),
            Line::from("v          - Enter history mode"),
            Line::from("Enter      - Select and exit"),
            Line::from("Esc        - Return to normal mode"),
            Line::from(""),
            Line::from("Mouse:"),
            Line::from("Click      - Select item"),
            Line::from("Double-click - Select and exit"),
            Line::from("Scroll     - Navigate results"),
        ];

        let help_items: Vec<ListItem> = help_content
            .into_iter()
            .map(ListItem::new)
            .collect();

        let help_widget = List::new(help_items)
            .block(Block::default().title("Search Help").borders(Borders::ALL));

        f.render_widget(help_widget, area);
    }
}
