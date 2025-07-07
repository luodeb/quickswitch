use anyhow::Result;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::{env, io};

use crate::app::App;
use crate::models::AppMode;

pub fn handle_key_event(app: &mut App, key: KeyCode) -> Result<bool> {
    match app.state.mode {
        AppMode::Normal => handle_normal_mode(app, key),
        AppMode::Search => handle_search_mode(app, key),
        AppMode::History => handle_history_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Esc => return Ok(false),
        KeyCode::Enter => {
            let selected_file = app.get_selected_file().cloned();
            handle_exit(app, selected_file.as_ref())?;
        }
        // k → 上
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(selected) = app.state.file_list_state.selected() {
                if selected > 0 {
                    app.state.file_list_state.select(Some(selected - 1));
                    app.update_preview();
                }
            }
        }
        // j → 下
        KeyCode::Char('j') | KeyCode::Down => {
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
        KeyCode::Char('l') | KeyCode::Right => {
            if let Some(file) = app.get_selected_file() {
                if file.is_dir {
                    app.change_directory(file.path.clone())?;
                }
            }
        }
        // h → 返回上级目录
        KeyCode::Char('h') | KeyCode::Left => {
            if let Some(parent) = app.state.current_dir.parent() {
                app.change_directory(parent.to_path_buf())?;
            }
        }
        // / → 进入搜索模式
        KeyCode::Char('/') => {
            app.enter_search_mode();
        }
        // H → 进入历史记录模式
        KeyCode::Char('v') => {
            app.enter_history_mode();
        }
        _ => {}
    }
    Ok(true)
}

fn handle_search_mode(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Esc => {
            app.enter_normal_mode();
        }
        KeyCode::Enter => {
            let selected_file = app.get_selected_file().cloned();
            handle_exit(app, selected_file.as_ref())?;
        }
        // 方向键导航（禁用 hjkl）
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
                    app.change_directory(file.path.clone())?;
                }
            }
        }
        KeyCode::Left => {
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

fn handle_history_mode(app: &mut App, key: KeyCode) -> Result<bool> {
    match key {
        KeyCode::Esc => {
            app.enter_normal_mode();
        }
        KeyCode::Enter => {
            if let Some(selected) = app.state.history_state.selected() {
                if let Some(path) = app.state.history.get(selected) {
                    let selected_path = path.clone();
                    // Move selected item to front
                    app.state.history.remove(selected);
                    app.state.history.insert(0, selected_path.clone());
                    app.save_history().unwrap_or(());
                    
                    // Change to selected directory
                    app.change_directory(selected_path)?;
                    app.enter_normal_mode();
                }
            }
        }
        // jk 或上下键导航历史记录
        KeyCode::Char('j') | KeyCode::Down => {
            if let Some(selected) = app.state.history_state.selected() {
                if selected < app.state.history.len() - 1 {
                    app.state.history_state.select(Some(selected + 1));
                }
            } else if !app.state.history.is_empty() {
                app.state.history_state.select(Some(0));
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(selected) = app.state.history_state.selected() {
                if selected > 0 {
                    app.state.history_state.select(Some(selected - 1));
                }
            }
        }
        // 禁用 hl 和左右键
        _ => {}
    }
    Ok(true)
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
