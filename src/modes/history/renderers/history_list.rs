use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, modes::shared::renderers::Renderer};

/// Renderer for history list in History mode
#[derive(Default)]
pub struct HistoryListRenderer;

impl HistoryListRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for HistoryListRenderer {
    fn render(&self, f: &mut Frame, area: Rect, app: &App) {
        let history_items: Vec<ListItem> = if app.state.filtered_history.is_empty() {
            if app.state.history.is_empty() {
                vec![ListItem::new("No history available")]
            } else {
                vec![ListItem::new("No matching history entries")]
            }
        } else {
            app.state
                .filtered_history
                .iter()
                .filter_map(|&i| app.state.history.get(i))
                .map(|path| {
                    let display_path = format!("📁 {}", path.display());
                    ListItem::new(display_path)
                })
                .collect()
        };

        let history_title = if app.state.is_searching && !app.state.search_input.is_empty() {
            format!(
                "History - {} matches ({}/{})",
                app.state.filtered_history.len(),
                app.state.filtered_history.len(),
                app.state.history.len()
            )
        } else {
            format!("History - {} entries", app.state.history.len())
        };

        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title(history_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(history_list, area, &mut app.state.history_state.clone());
    }
}
