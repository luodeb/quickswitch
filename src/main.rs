use quickswitch::{Result, ShellType, qs_init, run_interactive_mode, run_non_interactive, utils::AppMode};
use std::env;

fn print_help() {
    println!("quickswitch - A terminal-based tool for quickly switching between directories and files");
    println!();
    println!("USAGE:");
    println!("    quickswitch [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --mode <MODE>           Set the startup mode (normal, history) [default: normal]");
    println!("    --non-interactive       Run in non-interactive mode");
    println!("    --init <SHELL>          Initialize shell configuration (bash, zsh, fish, powershell, cmd)");
    println!("    --version               Print version information");
    println!("    --help, -h              Print this help message");
    println!();
    println!("MODES:");
    println!("    normal                  Default file navigation mode");
    println!("    history                 History selection mode");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut mode = AppMode::Normal; // Default mode
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--non-interactive" => {
                return run_non_interactive();
            }
            "--mode" => {
                // Parse mode parameter
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "normal" => mode = AppMode::Normal,
                        "history" => mode = AppMode::History,
                        _ => {
                            eprintln!("Error: Unsupported mode '{}'. Supported modes: normal, history", args[i + 1]);
                            std::process::exit(1);
                        }
                    }
                    i += 2; // Skip both --mode and its value
                    continue;
                } else {
                    eprintln!("Error: --mode requires a mode type (normal, history)");
                    std::process::exit(1);
                }
            }
            "--init" => {
                // Initialize the shell configuration
                if i + 1 < args.len() {
                    match args[i + 1].as_str() {
                        "bash" => return qs_init(ShellType::Bash),
                        "zsh" => return qs_init(ShellType::Zsh),
                        "fish" => return qs_init(ShellType::Fish),
                        "powershell" => return qs_init(ShellType::PowerShell),
                        "cmd" => return qs_init(ShellType::Cmd),
                        _ => {
                            eprintln!("Error: Unsupported shell type '{}'", args[i + 1]);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!(
                        "Error: --init requires a shell type (bash, zsh, fish, powershell, cmd)"
                    );
                    std::process::exit(1);
                }
            }
            "--version" => {
                let version = env!("CARGO_PKG_VERSION");
                println!("quickswitch version {version}");
                std::process::exit(0);
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                i += 1;
            }
        }
    }

    run_interactive_mode(mode).await
}
