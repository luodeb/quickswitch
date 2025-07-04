use quickswitch::{run_interactive_mode, run_non_interactive, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut output_file = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output-file" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --output-file requires a filename");
                    std::process::exit(1);
                }
            }
            "--non-interactive" => {
                return run_non_interactive();
            }
            _ => {
                i += 1;
            }
        }
    }

    run_interactive_mode(output_file).await
}
