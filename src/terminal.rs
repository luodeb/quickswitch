use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{fs::OpenOptions, io};

use crate::{app::App, events, ui, utils};

pub async fn run_interactive_mode(output_file: Option<String>) -> Result<()> {
    let terminal_result = if !utils::is_tty() {
        match OpenOptions::new().read(true).write(true).open("/dev/tty") {
            Ok(mut tty_file) => {
                enable_raw_mode()?;
                execute!(tty_file, EnterAlternateScreen, Clear(ClearType::All))?;
                let backend = CrosstermBackend::new(tty_file);
                let mut terminal = Terminal::new(backend)?;

                let mut app = App::new(output_file)?;
                let result = run_app_loop(&mut terminal, &mut app).await;

                disable_raw_mode()?;
                terminal.show_cursor()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

                Ok(result?)
            }
            Err(_) => utils::run_non_interactive(),
        }
    } else {
        let mut terminal = setup_terminal()?;
        let mut app = App::new(output_file)?;
        let result = run_app_loop(&mut terminal, &mut app).await;
        cleanup_terminal(&mut terminal)?;
        Ok(result?)
    };

    terminal_result
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
    app: &mut App,
) -> Result<()>
where
    W: std::io::Write,
{
    loop {
        terminal.draw(|f| ui::render_ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if !events::handle_key_event(app, key.code)? {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
