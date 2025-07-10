use anyhow::Result;
use crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, KeyCode, MouseEvent},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use ratatui::layout::Rect;
use std::{env, io};

use crate::{
    App, FileItem,
    modes::{ModeAction, history::HistoryDataProvider},
};

/// Main entry point for keyboard event handling
/// Now delegates to the app instead of handling directly
pub fn handle_key_event(app: &mut App, key: KeyCode) -> Result<bool> {
    let action = app.handle_key(key)?;
    match action {
        ModeAction::Stay => Ok(true),
        ModeAction::Switch(new_mode) => {
            app.switch_mode(new_mode)?;
            Ok(true)
        }
        ModeAction::Exit(file_item) => {
            handle_exit(app, file_item.as_ref())?;
            Ok(false) // This should never be reached due to process::exit in handle_exit
        }
    }
}

/// Handle mouse events
pub fn handle_mouse_event(
    app: &mut App,
    mouse: MouseEvent,
    left_area: Rect,
    right_area: Rect,
) -> Result<bool> {
    let action = app.handle_mouse(mouse, left_area, right_area)?;
    match action {
        ModeAction::Stay => Ok(true),
        ModeAction::Switch(new_mode) => {
            app.switch_mode(new_mode)?;
            Ok(true)
        }
        ModeAction::Exit(file_item) => {
            handle_exit(app, file_item.as_ref())?;
            Ok(false) // This should never be reached due to process::exit in handle_exit
        }
    }
}

pub fn handle_exit(app: &mut App, file: Option<&FileItem>) -> Result<()> {
    if let Some(file) = file {
        let select_path = if file.is_dir {
            file.path.clone()
        } else {
            app.state.current_dir.clone()
        };
        // Save to history using history data provider
        let history_provider: HistoryDataProvider = HistoryDataProvider;
        history_provider
            .add_to_history(select_path.clone())
            .unwrap_or(());

        // Properly cleanup terminal state before exit
        disable_raw_mode()?;
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        )?;

        unsafe { env::set_var("QS_SELECT_PATH", select_path.to_string_lossy().as_ref()) };
        eprintln!("{}", select_path.display());
    } else {
        // If no file is selected, just exit with proper cleanup
        disable_raw_mode()?;
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        )?;
    }

    std::process::exit(0);
}
