//! Console-level behavior of automatic highlighting (Rich's `highlight=True`).

use fast_rich::console::ColorSystem;
use fast_rich::highlighter::{Highlighter, ReprHighlighter};
use fast_rich::style::{Color, Style};
use fast_rich::text::{Span, Text};
use fast_rich::theme::Theme;
use fast_rich::Console;
use std::time::{Duration, Instant};

fn style_of(text: &Text, token: &str) -> Style {
    text.spans
        .iter()
        .find(|s| s.text == token)
        .unwrap_or_else(|| panic!("no span {token:?} in {:?}", text.spans))
        .style
}

#[test]
fn print_highlights_by_default() {
    let console = Console::capture();
    console.print("took 42 ms");
    let out = console.get_captured_output();
    assert!(out.contains("\x1b["), "expected ANSI styling in {out:?}");
    assert!(out.contains("42"));
}

#[test]
fn highlight_false_leaves_output_plain() {
    let console = Console::capture().highlight(false);
    console.print("took 42 ms, ok=True, url=https://x.io");
    assert_eq!(
        console.get_captured_output(),
        "took 42 ms, ok=True, url=https://x.io"
    );
}

#[test]
fn render_str_styles_tokens_like_rich() {
    let text = Console::capture().render_str("ip 10.0.0.1 n=7 'hi' None");
    assert_eq!(
        style_of(&text, "10.0.0.1"),
        Style::new().foreground(Color::BrightGreen).bold()
    );
    assert_eq!(style_of(&text, "n"), Style::new().foreground(Color::Yellow));
    assert_eq!(
        style_of(&text, "7"),
        Style::new().foreground(Color::Cyan).bold()
    );
    assert_eq!(
        style_of(&text, "'hi'"),
        Style::new().foreground(Color::Green)
    );
    assert_eq!(
        style_of(&text, "None"),
        Style::new().foreground(Color::Magenta).italic()
    );
}

#[test]
fn markup_wins_over_highlight() {
    // Rich: highlight first, markup on top -> "42" is bold (number) + red.
    let text = Console::capture().render_str("[red]42[/]");
    assert_eq!(
        style_of(&text, "42"),
        Style::new().foreground(Color::Red).bold()
    );
}

#[test]
fn markup_links_survive_highlight_splits() {
    let text = Console::capture().render_str("[link=https://e.com]id 42[/]");
    assert!(text
        .spans
        .iter()
        .all(|s| s.link.as_deref() == Some("https://e.com")));
    assert_eq!(text.plain_text(), "id 42");
}

#[test]
fn rust_literals_highlighted_by_default() {
    let text = Console::capture().render_str("ok: true, done: false");
    assert_eq!(
        style_of(&text, "true"),
        Style::new().foreground(Color::BrightGreen).italic()
    );
    assert_eq!(
        style_of(&text, "false"),
        Style::new().foreground(Color::BrightRed).italic()
    );
}

#[test]
fn rich_compatible_skips_rust_literals() {
    let console = Console::capture().highlighter(ReprHighlighter::new().rich_compatible());
    let text = console.render_str("ok true");
    assert!(text.spans.iter().all(|s| s.style.is_empty()));
}

#[test]
fn raw_print_is_never_highlighted() {
    let console = Console::capture();
    console.print_raw("[1, 2, 3] 42");
    assert_eq!(console.get_captured_output(), "[1, 2, 3] 42");
}

#[test]
fn renderables_are_not_highlighted() {
    let console = Console::capture();
    console.print_renderable(&Text::plain("42 True"));
    assert!(!console.get_captured_output().contains("\x1b["));
}

#[test]
fn markup_disabled_still_highlights() {
    // Rich highlights even with markup=False.
    let text = Console::capture().markup(false).render_str("[red] 42");
    assert_eq!(text.plain_text(), "[red] 42");
    assert!(style_of(&text, "[").bold);
    assert!(style_of(&text, "42").bold);
}

#[test]
fn theme_overrides_repr_styles() {
    let mut theme = Theme::new();
    theme.add_style("repr.number", Style::new().foreground(Color::Red));
    let text = Console::capture().theme(theme).render_str("n 5");
    assert_eq!(style_of(&text, "5"), Style::new().foreground(Color::Red));
}

#[test]
fn repr_styles_usable_as_markup() {
    let text = Console::capture()
        .highlight(false)
        .render_str("[repr.url]x[/]");
    assert_eq!(
        style_of(&text, "x"),
        Style::new().foreground(Color::BrightBlue).underline()
    );
}

#[test]
fn no_color_output_is_plain() {
    let console = Console::capture().color_system(ColorSystem::NoColor);
    console.print("n=42 'x' https://e.com");
    assert_eq!(console.get_captured_output(), "n=42 'x' https://e.com");
}

struct Shout;
impl Highlighter for Shout {
    fn highlight(&self, text: &str) -> Vec<Span> {
        text.split_inclusive('!')
            .map(|part| {
                if part.ends_with('!') {
                    Span::styled(part.to_string(), Style::new().bold())
                } else {
                    Span::raw(part.to_string())
                }
            })
            .collect()
    }
}

struct Broken;
impl Highlighter for Broken {
    fn highlight(&self, _text: &str) -> Vec<Span> {
        vec![Span::styled("nope", Style::new().bold())]
    }
}

#[test]
fn custom_highlighter_replaces_default() {
    let text = Console::capture().highlighter(Shout).render_str("hey! 42");
    assert!(style_of(&text, "hey!").bold);
    // The default number highlighting is gone.
    assert!(style_of(&text, " 42").is_empty());
}

#[test]
fn custom_highlighter_with_wrong_text_is_ignored() {
    let text = Console::capture()
        .highlighter(Broken)
        .render_str("hello 42");
    assert_eq!(text.plain_text(), "hello 42");
    assert!(text.spans.iter().all(|s| s.style.is_empty()));
}

#[test]
fn cloned_console_shares_custom_highlighter() {
    let console = Console::capture().highlighter(Shout);
    let clone = console.clone();
    assert!(style_of(&clone.render_str("a!"), "a!").bold);
}

#[test]
fn unicode_offsets_are_respected() {
    let text = Console::capture().render_str("é日本🎉 7 'ü'");
    assert_eq!(text.plain_text(), "é日本🎉 7 'ü'");
    assert!(style_of(&text, "7").bold);
    assert_eq!(
        style_of(&text, "'ü'"),
        Style::new().foreground(Color::Green)
    );
}

#[test]
fn deep_tokens_fit_small_stacks() {
    // Full ReprHighlighter (all passes) on a 256 KiB thread.
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let hl = ReprHighlighter::new();
            for input in [
                "/a".repeat(20_000),
                "(".repeat(20_000),
                "1.".repeat(20_000),
                "a:".repeat(20_000),
                format!("<{}>", "x=1 ".repeat(5_000)),
            ] {
                let joined: String = hl
                    .highlight(&input)
                    .iter()
                    .map(|s| s.text.as_ref())
                    .collect();
                assert_eq!(joined, input);
            }
        })
        .unwrap()
        .join()
        .expect("highlighter overflowed a 256 KiB stack");
}

#[test]
fn pathological_inputs_stay_fast() {
    // These are quadratic in Python Rich (seconds each); pruning guards keep
    // them linear here. Generous bound so debug builds on slow CI pass.
    let inputs = [
        "1".repeat(20_000),
        "a".repeat(20_000),
        "<".repeat(20_000),
        "/a".repeat(10_000),
        "'\\".repeat(10_000),
    ];
    let hl = ReprHighlighter::new();
    for input in &inputs {
        let start = Instant::now();
        let spans = hl.highlight(input);
        let joined: String = spans.iter().map(|s| s.text.as_ref()).collect();
        assert_eq!(&joined, input);
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "{:?}... took {:?}",
            &input[..8],
            start.elapsed()
        );
    }
}
