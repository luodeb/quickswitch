use clap::Parser;
use quickswitch::{
    Result, ShellType, qs_init, run_interactive_mode, run_non_interactive, utils::AppMode,
};

#[derive(Parser)]
#[command(
    name = "quickswitch",
    version,
    about = "A terminal-based tool for quickly switching between directories and files",
    long_about = None
)]
struct Cli {
    /// Set the startup mode
    #[arg(long, value_enum, default_value_t = AppMode::Normal)]
    mode: AppMode,

    /// Run in non-interactive mode
    #[arg(long)]
    non_interactive: bool,

    /// Initialize shell configuration (bash, zsh, fish, powershell, cmd)
    #[arg(long, value_enum)]
    init: Option<ShellType>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle init option
    if let Some(shell) = cli.init {
        return qs_init(shell);
    }

    // Handle non-interactive mode
    if cli.non_interactive {
        return run_non_interactive();
    }

    // Run interactive mode with specified mode
    run_interactive_mode(cli.mode).await
}
