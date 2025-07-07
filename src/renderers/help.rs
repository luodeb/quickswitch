use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for Normal mode help
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
            Line::from("V          - Enter history mode"),
            Line::from("Enter      - Select and exit"),
            Line::from("ESC        - Quit application"),
        ];

        let help_widget = List::new(
            help_content
                .into_iter()
                .map(|line| ListItem::new(line))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Normal Mode"),
        );

        f.render_widget(help_widget, area);
    }
}

/// Renderer for Search mode help
pub struct SearchHelpRenderer;

impl SearchHelpRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for SearchHelpRenderer {
    fn render(&self, f: &mut Frame, area: Rect, _app: &App) {
        let help_content = vec![
            Line::from("Search Mode Navigation:"),
            Line::from(""),
            Line::from("Type       - Search files"),
            Line::from("↑↓         - Navigate results"),
            Line::from("←→         - Navigate directories"),
            Line::from("Backspace  - Delete character"),
            Line::from(""),
            Line::from("Enter      - Select and exit"),
            Line::from("ESC        - Return to normal mode"),
            Line::from(""),
            Line::from("Note: hjkl keys are disabled in search mode"),
        ];

        let help_widget = List::new(
            help_content
                .into_iter()
                .map(|line| ListItem::new(line))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - Search Mode"),
        );

        f.render_widget(help_widget, area);
    }
}

/// Renderer for History mode help
pub struct HistoryHelpRenderer;

impl HistoryHelpRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for HistoryHelpRenderer {
    fn render(&self, f: &mut Frame, area: Rect, _app: &App) {
        let help_content = vec![
            Line::from("History Mode Navigation:"),
            Line::from(""),
            Line::from("j/k or ↑↓  - Navigate history"),
            Line::from("Enter      - Select directory"),
            Line::from("ESC        - Return to normal mode"),
            Line::from(""),
            Line::from("Note: Selected directory will be"),
            Line::from("      moved to top of history"),
        ];

        let help_widget = List::new(
            help_content
                .into_iter()
                .map(|line| ListItem::new(line))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help - History Mode"),
        );

        f.render_widget(help_widget, area);
    }
}
