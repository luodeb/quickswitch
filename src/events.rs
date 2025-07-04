use anyhow::Result;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::{env, io};

use crate::app::App;

pub fn handle_key_event(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Esc => return Ok(false),
        KeyCode::Enter => {
            // if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            //     // Ctrl+Enter 进入当前正在显示的文件夹（退出并返回当前目录）
            //     handle_exit(app, None)?;
            // } else {
            //     handle_exit(app, app.get_selected_file())?;
            // }
            handle_exit(app, app.get_selected_file())?;
        }
        KeyCode::Up => {
            if let Some(selected) = app.state.file_list_state.selected() {
                if selected > 0 {
                    app.state.file_list_state.select(Some(selected - 1));
                    app.update_preview();
                }
            }
        }
        KeyCode::Down => {
            if let Some(selected) = app.state.file_list_state.selected() {
                if selected < app.state.filtered_files.len() - 1 {
                    app.state.file_list_state.select(Some(selected + 1));
                    app.update_preview();
                }
            } else if !app.state.filtered_files.is_empty() {
                app.state.file_list_state.select(Some(0));
                app.update_preview();
            }
        }
        KeyCode::Right => {
            if let Some(file) = app.get_selected_file() {
                if file.is_dir {
                    app.state.current_dir = file.path.clone();
                    app.reload_directory()?;
                    app.state.search_input.clear();
                    app.update_filter();
                    app.state.file_list_state.select(Some(0));
                    app.update_preview();
                }
            }
        }
        KeyCode::Left => {
            if let Some(parent) = app.state.current_dir.parent() {
                app.state.current_dir = parent.to_path_buf();
                app.reload_directory()?;
                app.state.search_input.clear();
                app.update_filter();
                app.state.file_list_state.select(Some(0));
                app.update_preview();
            }
        }
        KeyCode::Backspace => {
            app.state.search_input.pop();
            app.update_filter();
            app.update_preview();
        }
        KeyCode::Char(c) => {
            app.state.search_input.push(c);
            app.update_filter();
            app.update_preview();
        }
        _ => {}
    }
    Ok(true)
}

pub fn handle_exit(app: &App, file: Option<&crate::models::FileItem>) -> Result<()> {
    let select_path = if let Some(file) = file {
        if file.is_dir {
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            format!("{}", file.path.display())
        } else {
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            format!("{}", app.state.current_dir.display())
        }
    } else {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        format!("{}", app.state.current_dir.display())
    };

    unsafe { env::set_var("QS_SELECT_PATH", &select_path) };

    if let Some(ref output_file) = app.state.output_file {
        if let Ok(mut file) = std::fs::File::create(output_file) {
            use std::io::Write;
            let _ = writeln!(file, "{}", select_path);
        }
    } else {
        println!("{}", select_path);
    }

    std::process::exit(0);
}
