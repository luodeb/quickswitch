use anyhow::Result;
use crossterm::{
    event::{KeyCode, MouseEvent, DisableMouseCapture},
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
    cursor::Show,
};
use ratatui::layout::Rect;
use std::{env, io};

use crate::modes::AppController;

/// Main entry point for keyboard event handling
/// Now delegates to the app controller instead of handling directly
pub fn handle_key_event(controller: &mut AppController, key: KeyCode) -> Result<bool> {
    controller.handle_key(key)
}

pub fn handle_exit(
    controller: &mut AppController,
    file: Option<&crate::models::FileItem>,
) -> Result<()> {
    let app = controller.get_app_mut();

    if let Some(file) = file {
        let select_path = if file.is_dir {
            file.path.clone()
        } else {
            app.state.current_dir.clone()
        };
        // Save to history
        app.add_to_history(select_path.clone()).unwrap_or(());

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

/// Handle mouse events
pub fn handle_mouse_event(
    controller: &mut AppController,
    mouse: MouseEvent,
    left_area: Rect,
    right_area: Rect,
) -> Result<bool> {
    controller.handle_mouse(mouse, left_area, right_area)
}
