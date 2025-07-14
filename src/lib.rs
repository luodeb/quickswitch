pub mod app;
pub mod app_state;
pub mod core;
pub mod modes;
pub mod preview_content;
pub mod services;
pub mod terminal;
pub mod utils;

pub use app::App;
pub use app_state::AppState;
pub use modes::ModeHandler;
pub use services::FilesystemService;
pub use terminal::run_interactive_mode;
pub use utils::{ShellType, is_tty, qs_init, run_non_interactive};

pub type Result<T> = anyhow::Result<T>;
