use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::io;

use crate::{App, events, models::AppMode};

pub async fn run_interactive_mode() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new(AppMode::Normal)?;
    let result = run_app_loop(&mut terminal, &mut app).await;
    cleanup_terminal(&mut terminal)?;
    result
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub async fn run_app_loop<W>(
    terminal: &mut Terminal<CrosstermBackend<W>>,
    app: &mut App,
) -> Result<()>
where
    W: std::io::Write,
{
    loop {
        // Calculate layout areas for mouse event handling
        let terminal_size = terminal.size()?;
        let terminal_area = Rect::new(0, 0, terminal_size.width, terminal_size.height);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(terminal_area);
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        let left_area = main_chunks[0];
        let right_area = main_chunks[1];

        terminal.draw(|f| render_ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press && !events::handle_key_event(app, key.code)?
                    {
                        break;
                    }
                }
                Event::Mouse(mouse) => {
                    if !events::handle_mouse_event(app, mouse, left_area, right_area)? {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// Simple UI rendering function that delegates to mode manager
fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Render search box
    let (title, content, style) = app.get_search_box_config();
    let search_box = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style);
    f.render_widget(search_box, chunks[0]);

    // Split main area for left and right panels
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Delegate rendering to app
    app.render_left_panel(f, main_chunks[0]);
    app.render_right_panel(f, main_chunks[1]);

    // Set cursor position when searching
    if app.state.is_searching {
        f.set_cursor_position((
            chunks[0].x + app.state.search_input.len() as u16 + 1,
            chunks[0].y + 1,
        ));
    }
}
