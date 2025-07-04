use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    env, fs, io::{self, Stdout}, path::PathBuf
};

struct App {
    search_input: String,
    current_dir: PathBuf,
    files: Vec<FileItem>,
    filtered_files: Vec<usize>,
    file_list_state: ListState,
    preview_content: Vec<Line<'static>>,
    preview_title: String,
}

#[derive(Clone)]
struct FileItem {
    name: String,
    path: PathBuf,
    is_dir: bool,
}

impl App {
    fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let mut app = Self {
            search_input: String::new(),
            current_dir: current_dir.clone(),
            files: Vec::new(),
            filtered_files: Vec::new(),
            file_list_state: ListState::default(),
            preview_content: Vec::new(),
            preview_title: String::new(),
        };
        app.load_directory()?;
        app.update_filter();
        app.file_list_state.select(Some(0));
        app.update_preview();
        Ok(app)
    }

    // ... 其他方法保持不变 ...
    fn load_directory(&mut self) -> Result<()> {
        self.files.clear();

        if let Some(parent) = self.current_dir.parent() {
            self.files.push(FileItem {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
            });
        }

        let entries = fs::read_dir(&self.current_dir)?;
        let mut items: Vec<FileItem> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                Some(FileItem { name, path, is_dir })
            })
            .collect();

        items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        self.files.extend(items);
        Ok(())
    }

    fn update_filter(&mut self) {
        if self.search_input.is_empty() {
            self.filtered_files = (0..self.files.len()).collect();
        } else {
            let search_lower = self.search_input.to_lowercase();
            self.filtered_files = self
                .files
                .iter()
                .enumerate()
                .filter(|(_, file)| file.name.to_lowercase().contains(&search_lower))
                .map(|(i, _)| i)
                .collect();
        }

        if !self.filtered_files.is_empty() {
            self.file_list_state.select(Some(0));
        } else {
            self.file_list_state.select(None);
        }
    }

    fn get_selected_file(&self) -> Option<&FileItem> {
        if let Some(selected) = self.file_list_state.selected() {
            if let Some(&file_index) = self.filtered_files.get(selected) {
                return self.files.get(file_index);
            }
        }
        None
    }

    fn update_preview(&mut self) {
        let selected_file_info = if let Some(file) = self.get_selected_file() {
            Some((file.name.clone(), file.path.clone(), file.is_dir))
        } else {
            None
        };

        if let Some((name, path, is_dir)) = selected_file_info {
            if is_dir {
                self.preview_title = format!("📁 {}", name);
                match fs::read_dir(&path) {
                    Ok(entries) => {
                        let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                        items.sort_by(|a, b| {
                            let a_is_dir = a.path().is_dir();
                            let b_is_dir = b.path().is_dir();
                            match (a_is_dir, b_is_dir) {
                                (true, false) => std::cmp::Ordering::Less,
                                (false, true) => std::cmp::Ordering::Greater,
                                _ => a.file_name().cmp(&b.file_name()),
                            }
                        });

                        self.preview_content = items
                            .iter()
                            .take(100)
                            .map(|entry| {
                                let name = entry.file_name().to_string_lossy().into_owned();
                                let is_dir = entry.path().is_dir();
                                let icon = if is_dir { "📁" } else { "📄" };
                                let style = if is_dir {
                                    Style::default().fg(Color::Cyan)
                                } else {
                                    Style::default()
                                };

                                Line::from(vec![
                                    Span::raw(icon.to_string()),
                                    Span::raw(" ".to_string()),
                                    Span::styled(name, style),
                                ])
                            })
                            .collect();

                        if items.len() > 100 {
                            self.preview_content.push(Line::from(vec![Span::styled(
                                format!("... and {} more items", items.len() - 100),
                                Style::default().fg(Color::Gray),
                            )]));
                        }

                        if self.preview_content.is_empty() {
                            self.preview_content.push(Line::from(vec![Span::styled(
                                "Empty directory".to_string(),
                                Style::default().fg(Color::Gray),
                            )]));
                        }
                    }
                    Err(e) => {
                        self.preview_content = vec![Line::from(vec![Span::styled(
                            format!("Error reading directory: {}", e),
                            Style::default().fg(Color::Red),
                        )])];
                    }
                }
            } else {
                self.preview_title = format!("📄 {}", name);
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        let size_info = Line::from(vec![Span::styled(
                            format!(
                                "Size: {} bytes, {} lines\n",
                                content.len(),
                                content.lines().count()
                            ),
                            Style::default().fg(Color::Gray),
                        )]);

                        let mut lines = vec![size_info];

                        lines.push(Line::from(vec![Span::styled(
                            "─".repeat(50),
                            Style::default().fg(Color::Gray),
                        )]));

                        let content_lines: Vec<Line<'static>> = content
                            .lines()
                            .take(100)
                            .enumerate()
                            .map(|(i, line)| {
                                Line::from(vec![
                                    Span::styled(
                                        format!("{:3} ", i + 1),
                                        Style::default().fg(Color::DarkGray),
                                    ),
                                    Span::raw(line.to_string()),
                                ])
                            })
                            .collect();

                        lines.extend(content_lines);

                        if content.lines().count() > 100 {
                            lines.push(Line::from(vec![Span::styled(
                                format!("... ({} more lines)", content.lines().count() - 100),
                                Style::default().fg(Color::Gray),
                            )]));
                        }

                        self.preview_content = lines;
                    }
                    Err(_) => match fs::metadata(&path) {
                        Ok(metadata) => {
                            self.preview_content = vec![
                                Line::from(vec![Span::styled(
                                    "Binary File".to_string(),
                                    Style::default().fg(Color::Yellow),
                                )]),
                                Line::from(vec![Span::raw("".to_string())]),
                                Line::from(vec![Span::styled(
                                    format!("Size: {} bytes", metadata.len()),
                                    Style::default().fg(Color::Gray),
                                )]),
                                Line::from(vec![Span::styled(
                                    "Cannot preview binary content".to_string(),
                                    Style::default().fg(Color::Gray),
                                )]),
                            ];
                        }
                        Err(e) => {
                            self.preview_content = vec![Line::from(vec![Span::styled(
                                format!("Error reading file: {}", e),
                                Style::default().fg(Color::Red),
                            )])];
                        }
                    },
                }
            }
        } else {
            self.preview_title = "Preview".to_string();
            self.preview_content = vec![Line::from(vec![Span::styled(
                "No file selected".to_string(),
                Style::default().fg(Color::Gray),
            )])];
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Esc => return Ok(false),
            KeyCode::Enter => {
                // Enter 退出程序并输出目录路径到 stdout
                if let Some(file) = self.get_selected_file() {
                    let select_path = if file.is_dir {
                        // 在清理终端之前输出路径
                        disable_raw_mode()?;
                        execute!(io::stdout(), LeaveAlternateScreen)?;
                        format!("{}", file.path.display())

                    } else {
                        disable_raw_mode()?;
                        execute!(io::stdout(), LeaveAlternateScreen)?;
                        format!("{}", self.current_dir.display())
                    };
                    unsafe { env::set_var("QS_SELECT_PATH", &select_path) };
                    println!("{}", select_path);
                    std::process::exit(0);
                } else {
                    disable_raw_mode()?;
                    execute!(io::stdout(), LeaveAlternateScreen)?;
                    println!("{}", self.current_dir.display());
                    std::process::exit(0);
                }
            }
            KeyCode::Up => {
                if let Some(selected) = self.file_list_state.selected() {
                    if selected > 0 {
                        self.file_list_state.select(Some(selected - 1));
                        self.update_preview();
                    }
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.file_list_state.selected() {
                    if selected < self.filtered_files.len() - 1 {
                        self.file_list_state.select(Some(selected + 1));
                        self.update_preview();
                    }
                } else if !self.filtered_files.is_empty() {
                    self.file_list_state.select(Some(0));
                    self.update_preview();
                }
            }
            KeyCode::Right => {
                if let Some(file) = self.get_selected_file() {
                    if file.is_dir {
                        self.current_dir = file.path.clone();
                        self.load_directory()?;
                        self.search_input.clear();
                        self.update_filter();
                        self.file_list_state.select(Some(0));
                        self.update_preview();
                    }
                }
            }
            KeyCode::Left => {
                if let Some(parent) = self.current_dir.parent() {
                    self.current_dir = parent.to_path_buf();
                    self.load_directory()?;
                    self.search_input.clear();
                    self.update_filter();
                    self.file_list_state.select(Some(0));
                    self.update_preview();
                }
            }
            KeyCode::Backspace => {
                self.search_input.pop();
                self.update_filter();
                self.update_preview();
            }
            KeyCode::Char(c) => {
                self.search_input.push(c);
                self.update_filter();
                self.update_preview();
            }
            _ => {}
        }
        Ok(true)
    }
}

// UI 函数保持不变
fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    let search_info = if app.search_input.is_empty() {
        format!("Search files (ESC to quit, Enter to exit & cd, ←→ navigate, ↑↓ select)")
    } else {
        format!(
            "Search: '{}' - {} matches",
            app.search_input,
            app.filtered_files.len()
        )
    };

    let search_box = Paragraph::new(app.search_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(search_info));
    f.render_widget(search_box, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let files: Vec<ListItem> = app
        .filtered_files
        .iter()
        .filter_map(|&i| app.files.get(i))
        .map(|file| {
            let icon = if file.is_dir { "📁" } else { "📄" };
            let style = if file.is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            let display_name = if !app.search_input.is_empty() {
                highlight_search_term(&file.name, &app.search_input)
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
        app.current_dir.display(),
        app.filtered_files.len(),
        app.files.len()
    );

    let files_list = List::new(files)
        .block(Block::default().borders(Borders::ALL).title(files_title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(files_list, main_chunks[0], &mut app.file_list_state.clone());

    let preview_list = List::new(
        app.preview_content
            .iter()
            .map(|line| ListItem::new(line.clone()))
            .collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(&*app.preview_title),
    );

    f.render_widget(preview_list, main_chunks[1]);

    f.set_cursor_position((
        chunks[0].x + app.search_input.len() as u16 + 1,
        chunks[0].y + 1,
    ));
}

fn highlight_search_term<'a>(text: &'a str, search: &'a str) -> Vec<Span<'a>> {
    if search.is_empty() {
        return vec![Span::raw(text)];
    }

    let search_lower = search.to_lowercase();
    let text_lower = text.to_lowercase();
    let mut spans = Vec::new();
    let mut last_end = 0;

    while let Some(start) = text_lower[last_end..].find(&search_lower) {
        let actual_start = last_end + start;
        let actual_end = actual_start + search.len();

        if actual_start > last_end {
            spans.push(Span::raw(&text[last_end..actual_start]));
        }

        spans.push(Span::styled(
            &text[actual_start..actual_end],
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ));

        last_end = actual_end;
    }

    if last_end < text.len() {
        spans.push(Span::raw(&text[last_end..]));
    }

    spans
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let result = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if !app.handle_key_event(key.code)? {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
