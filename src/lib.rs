pub mod app;
pub mod events;
pub mod filesystem;
pub mod models;
pub mod terminal;
pub mod ui;
pub mod utils;

pub use app::App;
pub use models::{AppState, FileItem};
pub use terminal::run_interactive_mode;
pub use utils::{is_tty, run_non_interactive};

pub type Result<T> = anyhow::Result<T>;
