use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::{fs::OpenOptions, io};

use crate::{events, models::AppMode, modes::AppController, utils};

pub async fn run_interactive_mode() -> Result<()> {
    if !utils::is_tty() {
        match OpenOptions::new().read(true).write(true).open("/dev/tty") {
            Ok(mut tty_file) => {
                enable_raw_mode()?;
                execute!(tty_file, EnterAlternateScreen, Clear(ClearType::All))?;
                let backend = CrosstermBackend::new(tty_file);
                let mut terminal = Terminal::new(backend)?;

                let mut controller = AppController::new(crate::models::AppMode::Normal)?;
                let result = run_app_loop(&mut terminal, &mut controller).await;

                disable_raw_mode()?;
                terminal.show_cursor()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                Ok(result?)
            }
            Err(_) => utils::run_non_interactive(),
        }
    } else {
        let mut terminal = setup_terminal()?;
        let mut controller = AppController::new(crate::models::AppMode::Normal)?;
        let result = run_app_loop(&mut terminal, &mut controller).await;
        cleanup_terminal(&mut terminal)?;
        Ok(result?)
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn cleanup_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub async fn run_app_loop<W>(
    terminal: &mut Terminal<CrosstermBackend<W>>,
    controller: &mut AppController,
) -> Result<()>
where
    W: std::io::Write,
{
    loop {
        terminal.draw(|f| render_ui(f, controller))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && !events::handle_key_event(controller, key.code)?
                {
                    break;
                }
            }
        }
    }
    Ok(())
}

/// Simple UI rendering function that delegates to mode manager
fn render_ui(f: &mut Frame, controller: &AppController) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Render search box
    let (title, content, style) = controller.get_search_box_config();
    let search_box = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style);
    f.render_widget(search_box, chunks[0]);

    // Split main area for left and right panels
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Delegate rendering to controller
    controller.render_left_panel(f, main_chunks[0]);
    controller.render_right_panel(f, main_chunks[1]);

    // Set cursor position for search mode
    if controller.is_mode(&AppMode::Search) {
        f.set_cursor_position((
            chunks[0].x + controller.get_app().state.search_input.len() as u16 + 1,
            chunks[0].y + 1,
        ));
    }
}
