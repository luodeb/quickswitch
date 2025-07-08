use anyhow::{Ok, Result};
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use std::io::IsTerminal;

pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
    Unknown,
}

pub fn is_tty() -> bool {
    std::io::stdin().is_terminal()
        && std::io::stdout().is_terminal()
        && std::io::stderr().is_terminal()
}

pub fn highlight_search_term<'a>(text: &'a str, search: &'a str) -> Vec<Span<'a>> {
    if search.is_empty() {
        return vec![Span::raw(text)];
    }

    let search_lower = search.to_lowercase();
    let text_lower = text.to_lowercase();
    let mut spans = Vec::new();
    let mut last_end = 0;

    while let Some(start) = text_lower[last_end..].find(&search_lower) {
        let actual_start = last_end + start;
        let actual_end = actual_start + search.len();

        if actual_start > last_end {
            spans.push(Span::raw(&text[last_end..actual_start]));
        }

        spans.push(Span::styled(
            &text[actual_start..actual_end],
            Style::default().fg(Color::Black).bg(Color::Yellow),
        ));

        last_end = actual_end;
    }

    if last_end < text.len() {
        spans.push(Span::raw(&text[last_end..]));
    }

    spans
}

pub fn run_non_interactive() -> Result<()> {
    println!("{}", std::env::current_dir()?.display());
    Ok(())
}

// Init Bash and Zsh functions for quickswitch
fn qs_init_bash_zsh() -> Result<()> {
    let bash_init = r#"
qs() {
    local dir
    dir=$(quickswitch "$PWD" 2>&1 >/dev/tty | tail -n 1)
    if [ -d "$dir" ]; then
        cd "$dir"
    fi
}
    "#;
    println!("{bash_init}");

    Ok(())
}

fn qs_init_fish() -> Result<()> {
    let fish_init = r#"
function qs
    set -l result (quickswitch "$PWD" 2>&1 >/dev/tty)

    if [ -n "$result" ]
        cd -- $result

        # Remove last token from commandline.
        commandline -t ""
        commandline -it -- $prefix
    end

    commandline -f repaint
end
    "#;
    println!("{fish_init}");

    Ok(())
}

fn qs_init_powershell() -> Result<()> {
    todo!("PowerShell initialization is not implemented yet");
}

fn qs_init_cmd() -> Result<()> {
    todo!("CMD initialization is not implemented yet");
}

pub fn qs_init(shell: ShellType) -> Result<()> {
    match shell {
        ShellType::Bash => qs_init_bash_zsh(),
        ShellType::Zsh => qs_init_bash_zsh(),
        ShellType::Fish => qs_init_fish(),
        ShellType::PowerShell => qs_init_powershell(),
        ShellType::Cmd => qs_init_cmd(),
        ShellType::Unknown => {
            eprintln!("Error: Unsupported shell type");
            std::process::exit(1);
        }
    }
}
