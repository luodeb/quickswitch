use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, modes::shared::renderers::Renderer, utils::DisplayItem};

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
        let history_items: Vec<ListItem> = if app.state.filtered_files.is_empty() {
            if app.state.files.is_empty() {
                vec![ListItem::new("No history available")]
            } else {
                vec![ListItem::new("No matching history entries")]
            }
        } else {
            app.state
                .filtered_files
                .iter()
                .filter_map(|&i| app.state.files.get(i))
                .map(|item| create_history_list_item(item, &app.state.search_input))
                .collect()
        };

        let history_title = if app.state.is_searching && !app.state.search_input.is_empty() {
            format!(
                "History - {} matches ({}/{})",
                app.state.filtered_files.len(),
                app.state.filtered_files.len(),
                app.state.files.len()
            )
        } else {
            format!("History - {} entries", app.state.files.len())
        };

        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title(history_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(history_list, area, &mut app.state.file_list_state.clone());
    }
}

/// Create a list item for a history entry with directory name and full path
fn create_history_list_item<'a>(item: &'a DisplayItem, search_input: &'a str) -> ListItem<'a> {
    match item {
        DisplayItem::HistoryPath(path) => {
            let icon = "📁";
            let dir_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let full_path = path.to_string_lossy();

            // Create spans for the display
            let mut spans = vec![
                Span::styled(icon, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
            ];

            // Add directory name with highlighting if searching
            if !search_input.is_empty() {
                let search_lower = search_input.to_lowercase();
                let name_lower = dir_name.to_lowercase();
                if let Some(pos) = name_lower.find(&search_lower) {
                    // Highlight the search term in directory name
                    let before = &dir_name[..pos];
                    let matched = &dir_name[pos..pos + search_input.len()];
                    let after = &dir_name[pos + search_input.len()..];

                    spans.push(Span::styled(before, Style::default().fg(Color::Cyan)));
                    spans.push(Span::styled(
                        matched,
                        Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                    ));
                    spans.push(Span::styled(after, Style::default().fg(Color::Cyan)));
                } else {
                    spans.push(Span::styled(dir_name, Style::default().fg(Color::Cyan)));
                }
            } else {
                spans.push(Span::styled(dir_name, Style::default().fg(Color::Cyan)));
            }

            // Add full path in darker color
            spans.push(Span::raw("  "));
            spans.push(Span::styled(
                format!("({})", full_path),
                Style::default().fg(Color::DarkGray),
            ));

            ListItem::new(Line::from(spans))
        }
        DisplayItem::File(_) => {
            // This shouldn't happen in history mode, but handle it gracefully
            ListItem::new("Invalid history entry")
        }
    }
}
