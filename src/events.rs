use anyhow::Result;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::{env, io};

use crate::app::App;

/// Main entry point for keyboard event handling
/// Now delegates to the mode manager instead of handling directly
pub fn handle_key_event(app: &mut App, key: KeyCode) -> Result<bool> {
    // Temporarily take mode_manager out to avoid borrowing conflicts
    if let Some(mut mode_manager) = app.mode_manager.take() {
        let result = mode_manager.handle_key(app, key);
        // Put mode_manager back
        app.mode_manager = Some(mode_manager);
        result
    } else {
        // Fallback if mode manager is not initialized
        Ok(false)
    }
}

pub fn handle_exit(app: &mut App, file: Option<&crate::models::FileItem>) -> Result<()> {
    let select_path = if let Some(file) = file {
        if file.is_dir {
            file.path.clone()
        } else {
            app.state.current_dir.clone()
        }
    } else {
        app.state.current_dir.clone()
    };

    // Save to history
    app.add_to_history(select_path.clone()).unwrap_or(());

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    unsafe { env::set_var("QS_SELECT_PATH", select_path.to_string_lossy().as_ref()) };
    eprintln!("{}", select_path.display());

    std::process::exit(0);
}
