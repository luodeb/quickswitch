use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{app::App, utils};

pub fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    render_search_box(f, chunks[0], app);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_file_list(f, main_chunks[0], app);
    render_preview(f, main_chunks[1], app);

    f.set_cursor_position((
        chunks[0].x + app.state.search_input.len() as u16 + 1,
        chunks[0].y + 1,
    ));
}

pub fn render_search_box(f: &mut Frame, area: Rect, app: &App) {
    let search_info = if app.state.search_input.is_empty() {
        format!("Search files (ESC to quit, Enter to exit & cd, ←→ navigate, ↑↓ select)")
    } else {
        format!(
            "Search: '{}' - {} matches",
            app.state.search_input,
            app.state.filtered_files.len()
        )
    };

    let search_box = Paragraph::new(app.state.search_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(search_info));
    f.render_widget(search_box, area);
}

pub fn render_file_list(f: &mut Frame, area: Rect, app: &App) {
    let files: Vec<ListItem> = app
        .state
        .filtered_files
        .iter()
        .filter_map(|&i| app.state.files.get(i))
        .map(|file| {
            let icon = if file.is_dir { "📁" } else { "📄" };
            let style = if file.is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            let display_name = if !app.state.search_input.is_empty() {
                utils::highlight_search_term(&file.name, &app.state.search_input)
            } else {
                vec![Span::styled(&file.name, style)]
            };

            let mut spans = vec![Span::raw(icon), Span::raw(" ")];
            spans.extend(display_name);

            ListItem::new(Line::from(spans))
        })
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

pub fn render_preview(f: &mut Frame, area: Rect, app: &App) {
    let preview_list = List::new(
        app.state
            .preview_content
            .iter()
            .map(|line| ListItem::new(line.clone()))
            .collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(&*app.state.preview_title),
    );

    f.render_widget(preview_list, area);
}
