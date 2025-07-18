use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    AppState,
    modes::Renderer,
    utils::{self, DisplayItem, FileItem},
};

/// Renderer for file list in Normal mode
#[derive(Default)]
pub struct FileListRenderer;

impl FileListRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for FileListRenderer {
    fn render(&self, f: &mut Frame, area: Rect, state: &AppState) {
        let files: Vec<ListItem> = state
            .filtered_files
            .iter()
            .filter_map(|&i| state.files.get(i))
            .map(|item| create_display_item_list_item(item, &state.search_input))
            .collect();

        let files_title = format!(
            "Files - {} ({}/{})",
            state.current_dir.display(),
            state.filtered_files.len(),
            state.files.len()
        );

        let files_list = List::new(files)
            .block(Block::default().borders(Borders::ALL).title(files_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(files_list, area, &mut state.file_list_state.clone());
    }
}

/// Create a list item for a file with optional search highlighting
fn create_file_list_item<'a>(file: &'a FileItem, search_input: &'a str) -> ListItem<'a> {
    let icon = if file.is_dir { "📁" } else { "📄" };
    let style = if file.is_dir {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let display_name = if !search_input.is_empty() {
        utils::highlight_search_term(&file.name, search_input)
    } else {
        vec![Span::styled(&file.name, style)]
    };

    let mut spans = vec![Span::raw(icon), Span::raw(" ")];
    spans.extend(display_name);

    ListItem::new(Line::from(spans))
}

/// Create a list item for a DisplayItem with optional search highlighting
fn create_display_item_list_item<'a>(item: &'a DisplayItem, search_input: &'a str) -> ListItem<'a> {
    match item {
        DisplayItem::File(file) => create_file_list_item(file, search_input),
        DisplayItem::History(entry) => {
            let icon = "📁";
            let style = Style::default().fg(Color::Cyan);
            let name = entry
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            let display_name = if !search_input.is_empty() {
                utils::highlight_search_term(name, search_input)
            } else {
                vec![Span::styled(name, style)]
            };

            let mut spans = vec![Span::raw(icon), Span::raw(" ")];
            spans.extend(display_name);

            ListItem::new(Line::from(spans))
        }
    }
}
