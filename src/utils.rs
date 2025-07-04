use anyhow::Result;
use ratatui::{
    style::{Color, Style},
    text::Span,
};
use std::io::IsTerminal;

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
