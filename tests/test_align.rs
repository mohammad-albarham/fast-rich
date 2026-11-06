//! Integration tests for align wrapper

use fast_rich::align::{Align, VerticalAlignment};
use fast_rich::console::RenderContext;
use fast_rich::renderable::Renderable;
use fast_rich::text::Text;

#[test]
fn test_align_center() {
    let text = Text::plain("Hello");
    let aligned = Align::center(text);

    let context = RenderContext {
        width: 20,
        height: None,
    };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_align_left() {
    let text = Text::plain("Test");
    let aligned = Align::left(text);

    let context = RenderContext {
        width: 20,
        height: None,
    };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_align_right() {
    let text = Text::plain("Right");
    let aligned = Align::right(text);

    let context = RenderContext {
        width: 20,
        height: None,
    };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_vertical_alignment() {
    let text = Text::plain("Vertical");
    let aligned = Align::center(text)
        .vertical(VerticalAlignment::Middle)
        .height(5);

    let context = RenderContext {
        width: 20,
        height: None,
    };
    let segments = aligned.render(&context);

    // Should have more segments due to vertical padding
    assert!(segments.len() >= 3); // empty lines + content + empty lines
}

fn plain(r: &dyn fast_rich::renderable::Renderable, width: usize) -> String {
    fast_rich::renderable::render_lines(r, width, fast_rich::style::Style::new())
        .iter()
        .map(|l| l.iter().map(|s| s.text.as_ref()).collect::<String>())
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_align_matches_rich() {
    use fast_rich::constrain::Constrain;
    use fast_rich::padding::{Padding, PaddingSpec};
    use fast_rich::panel::Panel;
    use fast_rich::text::Text;

    // Multi-line content is aligned as one block, like Rich.
    let centered = Align::center(Panel::new("hi\nthere"));
    assert_eq!(
        plain(&centered, 30),
        "          ╭───────╮\n          │ hi    │\n          │ there │\n          ╰───────╯"
    );
    assert_eq!(
        plain(&Align::right(Text::plain("end")), 30),
        format!("{}end", " ".repeat(27))
    );
    let padded = Padding::new(Text::plain("abc def"), PaddingSpec::symmetric(1, 2));
    assert_eq!(plain(&padded, 12), "\n  abc def\n");
    let constrained = Constrain::new(Text::plain("aaa bbb ccc ddd"), Some(8));
    assert_eq!(plain(&constrained, 30), "aaa bbb\nccc ddd");
}
