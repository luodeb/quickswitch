use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::App, renderers::Renderer, utils};

/// Renderer for file list in Normal and Search modes
#[derive(Default)]
pub struct FileListRenderer;

impl FileListRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for FileListRenderer {
    fn render(&self, f: &mut Frame, area: Rect, app: &App) {
        let files: Vec<ListItem> = app
            .state
            .filtered_files
            .iter()
            .filter_map(|&i| app.state.files.get(i))
            .map(|file| create_file_list_item(file, &app.state.search_input))
            .collect();

        let files_title = format!(
            "Files - {} ({}/{})",
            app.state.current_dir.display(),
            app.state.filtered_files.len(),
            app.state.files.len()
        );

        let files_list = List::new(files)
            .block(Block::default().borders(Borders::ALL).title(files_title))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(files_list, area, &mut app.state.file_list_state.clone());
    }
}

/// Create a list item for a file with optional search highlighting
fn create_file_list_item<'a>(
    file: &'a crate::models::FileItem,
    search_input: &'a str,
) -> ListItem<'a> {
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
