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
        handle_exit(app, app.get_selected_file())?;
    }
    // k → 上
    KeyCode::Char('k') => {
        if let Some(selected) = app.state.file_list_state.selected() {
            if selected > 0 {
                app.state.file_list_state.select(Some(selected - 1));
                app.update_preview();
            }
        }
    }
    // j → 下
    KeyCode::Char('j') => {
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
    // l → 进入文件夹
    KeyCode::Char('l') => {
        if let Some(file) = app.get_selected_file() {
            if file.is_dir {
                app.change_directory(file.path.clone())?;
            }
        }
    }
    // h → 返回上级目录
    KeyCode::Char('h') => {
        if let Some(parent) = app.state.current_dir.parent() {
            app.change_directory(parent.to_path_buf())?;
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
        // use stderr to convert message
        eprintln!("{}", select_path);
    }

    std::process::exit(0);
}
