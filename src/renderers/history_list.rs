use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer};

/// Renderer for history list in History mode
pub struct HistoryListRenderer;

impl HistoryListRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for HistoryListRenderer {
    fn render(&self, f: &mut Frame, area: Rect, app: &App) {
        let history_items: Vec<ListItem> = if app.state.history.is_empty() {
            vec![ListItem::new("No history available")]
        } else {
            app.state
                .history
                .iter()
                .map(|path| {
                    let display_path = format!("📁 {}", path.display());
                    ListItem::new(display_path)
                })
                .collect()
        };

        let history_title = format!("History - {} entries", app.state.history.len());

        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title(history_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(history_list, area, &mut app.state.history_state.clone());
    }
}
