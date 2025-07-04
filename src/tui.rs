use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let height = 10;
            // Ensure the UI is at the bottom of the terminal.
            let ui_area = Rect::new(
                area.x,
                area.y + area.height.saturating_sub(height),
                area.width,
                height,
            );

            // Use the ratatui's Clear widget to clear the area.
            frame.render_widget(Clear, ui_area);

            let block = Block::default()
                .title(" quickswitch ")
                .borders(Borders::TOP);
            frame.render_widget(block, ui_area);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    // When exiting, we leave the last frame on the screen and the cursor below it.
    // The shell prompt will then appear on a new line.
    println!();
    Ok(())
}
