use rich_rust::prelude::*;
use rich_rust::highlighter::{RegexHighlighter, Highlighter, highlight_text};

fn main() {
    let console = Console::new().width(70);
    console.rule("[bold magenta]Highlighter Demo[/]");

    // Example 1: Number highlighter
    console.print("[bold]Number Highlighter:[/]");
    let number_hl = RegexHighlighter::number_highlighter(
        Style::new().foreground(Color::Cyan).bold()
    );
    let text1 = "Server started on port 8080 with 256MB RAM";
    let highlighted1 = highlight_text(text1, &number_hl);
    console.print_renderable(&highlighted1);
    console.newline();

    // Example 2: URL highlighter
    console.print("[bold]URL Highlighter:[/]");
    let url_hl = RegexHighlighter::url_highlighter(
        Style::new().foreground(Color::Blue).underline()
    );
    let text2 = "Visit https://example.com or http://rust-lang.org for more info";
    let highlighted2 = highlight_text(text2, &url_hl);
    console.print_renderable(&highlighted2);
    console.newline();

    // Example 3: Email highlighter
    console.print("[bold]Email Highlighter:[/]");
    let email_hl = RegexHighlighter::email_highlighter(
        Style::new().foreground(Color::Green)
    );
    let text3 = "Contact us at support@example.com or admin@test.org";
    let highlighted3 = highlight_text(text3, &email_hl);
    console.print_renderable(&highlighted3);
    console.newline();

    // Example 4: Custom pattern
    console.print("[bold]Custom Pattern (words starting with capital):[/]");
    let mut custom_hl = RegexHighlighter::new();
    custom_hl.add_pattern(
        r"\b[A-Z][a-z]+\b",
        Style::new().foreground(Color::Yellow).bold()
    ).unwrap();
    let text4 = "The Quick Brown Fox jumps over the Lazy Dog";
    let highlighted4 = highlight_text(text4, &custom_hl);
    console.print_renderable(&highlighted4);
    console.newline();

    // Example 5: Multiple patterns
    console.print("[bold]Multiple Patterns (numbers + URLs):[/]");
    let mut multi_hl = RegexHighlighter::new();
    multi_hl.add_pattern(r"\d+", Style::new().foreground(Color::Cyan)).unwrap();
    multi_hl.add_pattern(r"https?://[^\s]+", Style::new().foreground(Color::Blue).underline()).unwrap();
    let text5 = "Server 1 at https://api.example.com handles 1000 requests/sec";
    let highlighted5 = highlight_text(text5, &multi_hl);
    console.print_renderable(&highlighted5);

    console.rule("[bold magenta]End Demo[/]");
}
