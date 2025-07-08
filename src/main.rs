use quickswitch::{Result, ShellType, qs_init, run_interactive_mode, run_non_interactive};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--non-interactive" => {
                return run_non_interactive();
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
            _ => {
                i += 1;
            }
        }
    }

    run_interactive_mode().await
}
