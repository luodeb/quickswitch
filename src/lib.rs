pub mod app;
pub mod events;
pub mod models;
pub mod terminal;
pub mod utils;
pub mod modes;
pub mod handlers;
pub mod renderers;
pub mod services;

pub use app::App;
pub use models::{AppState, FileItem};
pub use terminal::run_interactive_mode;
pub use utils::{is_tty, run_non_interactive, qs_init, ShellType};
pub use modes::{ModeManager, ModeHandler};
pub use services::FilesystemService;

pub type Result<T> = anyhow::Result<T>;
