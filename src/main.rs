use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{
    fs,
    io::{self, Stdout},
    path::{Path, PathBuf},
};

struct App {
    input: String,
    current_dir: PathBuf,
    files: Vec<FileItem>,
    file_list_state: ListState,
    preview_content: String,
    input_mode: bool,
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
            input: String::new(),
            current_dir: current_dir.clone(),
            files: Vec::new(),
            file_list_state: ListState::default(),
            preview_content: String::new(),
            input_mode: true, // 默认启用输入模式
        };
        app.load_directory()?;
        app.file_list_state.select(Some(0));
        app.update_preview();
        Ok(app)
    }

    fn load_directory(&mut self) -> Result<()> {
        self.files.clear();

        // 添加父目录选项（除非是根目录）
        if let Some(parent) = self.current_dir.parent() {
            self.files.push(FileItem {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
            });
        }

        // 读取当前目录内容
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

        // 排序：目录在前，文件在后，按名称排序
        items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        self.files.extend(items);
        Ok(())
    }

    fn update_preview(&mut self) {
        if let Some(selected) = self.file_list_state.selected() {
            if let Some(file) = self.files.get(selected) {
                if file.is_dir {
                    // 预览目录内容
                    match fs::read_dir(&file.path) {
                        Ok(entries) => {
                            let mut content = format!("Directory: {}\n\nContents:\n", file.name);
                            let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                            items.sort_by_key(|e| e.file_name());

                            for entry in items.iter().take(20) {
                                let name = entry.file_name().to_string_lossy().to_string();
                                let is_dir = entry.path().is_dir();
                                content.push_str(&format!(
                                    "{} {}\n",
                                    if is_dir { "[DIR]" } else { "[FILE]" },
                                    name
                                ));
                            }

                            if items.len() > 20 {
                                content
                                    .push_str(&format!("... and {} more items", items.len() - 20));
                            }

                            self.preview_content = content;
                        }
                        Err(e) => {
                            self.preview_content = format!("Error reading directory: {}", e);
                        }
                    }
                } else {
                    // 预览文件内容
                    match fs::read_to_string(&file.path) {
                        Ok(content) => {
                            let lines: Vec<&str> = content.lines().take(50).collect();
                            self.preview_content = format!(
                                "File: {}\nSize: {} bytes\n\nContent:\n{}{}",
                                file.name,
                                content.len(),
                                lines.join("\n"),
                                if content.lines().count() > 50 {
                                    "\n\n... (truncated)"
                                } else {
                                    ""
                                }
                            );
                        }
                        Err(_) => {
                            // 可能是二进制文件
                            match fs::metadata(&file.path) {
                                Ok(metadata) => {
                                    self.preview_content = format!(
                                        "File: {}\nSize: {} bytes\nType: Binary file (cannot preview)",
                                        file.name,
                                        metadata.len()
                                    );
                                }
                                Err(e) => {
                                    self.preview_content = format!("Error reading file: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') if !self.input_mode => return Ok(false),
            KeyCode::Esc => {
                if self.input_mode {
                    self.input_mode = false;
                    self.input.clear();
                } else {
                    return Ok(false); // 如果不在输入模式，ESC 退出程序
                }
            }
            KeyCode::Tab => {
                // Tab 键切换输入模式
                self.input_mode = !self.input_mode;
                if !self.input_mode {
                    self.input.clear();
                }
            }
            _ => {
                if self.input_mode {
                    match key {
                        KeyCode::Enter => {
                            if !self.input.is_empty() {
                                // 处理输入命令
                                if self.input.starts_with("cd ") {
                                    let path = self.input[3..].trim();
                                    let new_path = if path.starts_with('/') {
                                        PathBuf::from(path)
                                    } else {
                                        self.current_dir.join(path)
                                    };

                                    if new_path.exists() && new_path.is_dir() {
                                        self.current_dir = new_path;
                                        self.load_directory()?;
                                        self.file_list_state.select(Some(0));
                                        self.update_preview();
                                    }
                                }
                                self.input.clear();
                            } else {
                                // 如果输入框为空，Enter 键进入选中的目录
                                if let Some(selected) = self.file_list_state.selected() {
                                    if let Some(file) = self.files.get(selected) {
                                        if file.is_dir {
                                            self.current_dir = file.path.clone();
                                            self.load_directory()?;
                                            self.file_list_state.select(Some(0));
                                            self.update_preview();
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Up => {
                            // 在输入模式下也可以使用上下键
                            if let Some(selected) = self.file_list_state.selected() {
                                if selected > 0 {
                                    self.file_list_state.select(Some(selected - 1));
                                    self.update_preview();
                                }
                            }
                        }
                        KeyCode::Down => {
                            // 在输入模式下也可以使用上下键
                            if let Some(selected) = self.file_list_state.selected() {
                                if selected < self.files.len() - 1 {
                                    self.file_list_state.select(Some(selected + 1));
                                    self.update_preview();
                                }
                            } else if !self.files.is_empty() {
                                self.file_list_state.select(Some(0));
                                self.update_preview();
                            }
                        }
                        _ => {}
                    }
                } else {
                    // 非输入模式的按键处理
                    match key {
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
                                if selected < self.files.len() - 1 {
                                    self.file_list_state.select(Some(selected + 1));
                                    self.update_preview();
                                }
                            } else if !self.files.is_empty() {
                                self.file_list_state.select(Some(0));
                                self.update_preview();
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = self.file_list_state.selected() {
                                if let Some(file) = self.files.get(selected) {
                                    if file.is_dir {
                                        self.current_dir = file.path.clone();
                                        self.load_directory()?;
                                        self.file_list_state.select(Some(0));
                                        self.update_preview();
                                    }
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            // 在非输入模式下输入字符会自动启用输入模式
                            self.input_mode = true;
                            self.input.push(c);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(true)
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // 输入框
    let input_style = if app.input_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let title = if app.input_mode {
        "Command (Tab to switch mode, ESC to clear/exit)"
    } else {
        "Navigation Mode (Tab to switch, ESC/q to quit)"
    };

    let input = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(input, chunks[0]);

    // 主要区域分为左右两部分
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // 左侧文件列表
    let files: Vec<ListItem> = app
        .files
        .iter()
        .map(|file| {
            let icon = if file.is_dir { "📁" } else { "📄" };
            let style = if file.is_dir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::raw(icon),
                Span::raw(" "),
                Span::styled(&file.name, style),
            ]))
        })
        .collect();

    let files_list = List::new(files)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Files - {}", app.current_dir.display())),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(files_list, main_chunks[0], &mut app.file_list_state.clone());

    // 右侧预览区
    let preview = Paragraph::new(app.preview_content.as_str())
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(preview, main_chunks[1]);

    // 如果在输入模式，设置光标位置
    if app.input_mode {
        f.set_cursor_position((chunks[0].x + app.input.len() as u16 + 1, chunks[0].y + 1));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = App::new()?;

    // 主循环
    let result = run_app(&mut terminal, &mut app).await;

    // 清理
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("Error: {}", err);
    }

    Ok(())
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
