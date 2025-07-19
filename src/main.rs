use clap::Parser;
use quickswitch::{
    Result, ShellType, logging::init_logging, qs_init, run_interactive_mode, run_non_interactive,
    utils::AppMode,
};
use std::path::PathBuf;

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

    /// Enable verbose logging (-v=INFO, -vv=DEBUG, -vvv=TRACE)
    #[arg(short = 'v', action = clap::ArgAction::Count)]
    verbose: u8,

    /// Log file path (creates temp file `qw-[date]-[pid].log` if not specified)
    #[arg(long)]
    log_file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging if verbose flag is set
    init_logging(cli.verbose, cli.log_file.as_deref())?;

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
