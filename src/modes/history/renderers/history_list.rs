use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{AppState, modes::Renderer, utils::DisplayItem};

/// Renderer for history list in History mode
#[derive(Default)]
pub struct HistoryListRenderer;

impl HistoryListRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for HistoryListRenderer {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let history_items: Vec<ListItem> = if state.filtered_files.is_empty() {
            if state.files.is_empty() {
                vec![ListItem::new("No history available")]
            } else {
                vec![ListItem::new("No matching history entries")]
            }
        } else {
            state
                .filtered_files
                .iter()
                .filter_map(|&i| state.files.get(i))
                .map(|item| create_history_list_item(item, &state.search_input))
                .collect()
        };

        let history_title = if state.is_searching && !state.search_input.is_empty() {
            format!(
                "History - {} matches ({}/{})",
                state.filtered_files.len(),
                state.filtered_files.len(),
                state.files.len()
            )
        } else {
            format!("History - {} entries", state.files.len())
        };

        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title(history_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(history_list, area, &mut state.file_list_state.clone());
    }
}

/// Create a list item for a history entry with directory name and full path
fn create_history_list_item<'a>(item: &'a DisplayItem, search_input: &'a str) -> ListItem<'a> {
    match item {
        DisplayItem::History(entry) => {
            let icon = "📁";
            let dir_name = entry
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let full_path = entry.path.to_string_lossy();

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

            // Add frequency indicator
            spans.push(Span::styled(
                format!(" ({}×)", entry.frequency),
                Style::default().fg(Color::Yellow),
            ));

            // Add full path in darker color
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                format!("({full_path})"),
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
