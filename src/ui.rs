use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{app::App, utils, models::AppMode};

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

    render_left_panel(f, main_chunks[0], app);
    render_right_panel(f, main_chunks[1], app);

    // Only show cursor in search mode
    if app.state.mode == AppMode::Search {
        f.set_cursor_position((
            chunks[0].x + app.state.search_input.len() as u16 + 1,
            chunks[0].y + 1,
        ));
    }
}

fn render_left_panel(f: &mut Frame, area: Rect, app: &App) {
    match app.state.mode {
        AppMode::History => render_history_list(f, area, app),
        _ => render_file_list(f, area, app),
    }
}

fn render_right_panel(f: &mut Frame, area: Rect, app: &App) {
    match app.state.mode {
        AppMode::Normal => {
            if should_show_normal_help(app) {
                render_normal_help(f, area);
            } else {
                render_preview(f, area, app);
            }
        }
        AppMode::Search => {
            if should_show_search_help(app) {
                render_search_help(f, area);
            } else {
                render_preview(f, area, app);
            }
        }
        AppMode::History => {
            render_history_help(f, area);
        }
    }
}

fn should_show_normal_help(app: &App) -> bool {
    app.state.file_list_state.selected().is_none() || app.state.filtered_files.is_empty()
}

fn should_show_search_help(app: &App) -> bool {
    app.state.search_input.is_empty() || app.state.filtered_files.is_empty()
}

fn render_normal_help(f: &mut Frame, area: Rect) {
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
        help_content.into_iter().map(|line| ListItem::new(line)).collect::<Vec<_>>()
    )
    .block(Block::default().borders(Borders::ALL).title("Help - Normal Mode"));
    
    f.render_widget(help_widget, area);
}

fn render_search_help(f: &mut Frame, area: Rect) {
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
        help_content.into_iter().map(|line| ListItem::new(line)).collect::<Vec<_>>()
    )
    .block(Block::default().borders(Borders::ALL).title("Help - Search Mode"));
    
    f.render_widget(help_widget, area);
}

fn render_history_help(f: &mut Frame, area: Rect) {
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
        help_content.into_iter().map(|line| ListItem::new(line)).collect::<Vec<_>>()
    )
    .block(Block::default().borders(Borders::ALL).title("Help - History Mode"));
    
    f.render_widget(help_widget, area);
}

pub fn render_search_box(f: &mut Frame, area: Rect, app: &App) {
    let (search_info, search_content, search_style) = get_search_box_config(app);
    
    let search_box = Paragraph::new(search_content.as_str())
        .style(search_style)
        .block(Block::default().borders(Borders::ALL).title(search_info));
    f.render_widget(search_box, area);
}

fn get_search_box_config(app: &App) -> (String, String, Style) {
    match app.state.mode {
        AppMode::Normal => {
            let info = if app.state.search_input.is_empty() {
                "NORMAL - hjkl navigate, / search, V history, Enter exit".to_string()
            } else {
                format!("NORMAL - Search: '{}' - {} matches", 
                    app.state.search_input, 
                    app.state.filtered_files.len())
            };
            (info, app.state.search_input.clone(), Style::default().fg(Color::Yellow))
        }
        AppMode::Search => {
            let info = if app.state.search_input.is_empty() {
                "SEARCH - Type to search, ESC to normal mode".to_string()
            } else {
                format!("SEARCH - '{}' - {} matches (ESC to normal)", 
                    app.state.search_input, 
                    app.state.filtered_files.len())
            };
            (info, app.state.search_input.clone(), Style::default().fg(Color::Black).bg(Color::Yellow))
        }
        AppMode::History => {
            let info = format!("HISTORY - {} entries (jk navigate, Enter select, ESC to normal)", 
                app.state.history.len());
            (info, String::new(), Style::default().fg(Color::Cyan))
        }
    }
}

pub fn render_file_list(f: &mut Frame, area: Rect, app: &App) {
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

fn create_file_list_item<'a>(file: &'a crate::models::FileItem, search_input: &'a str) -> ListItem<'a> {
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

pub fn render_history_list(f: &mut Frame, area: Rect, app: &App) {
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
