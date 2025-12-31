//! Integration tests for align wrapper

use fast_rich::align::{Align, VerticalAlignment};
use fast_rich::console::RenderContext;
use fast_rich::renderable::Renderable;
use fast_rich::text::Text;

#[test]
fn test_align_center() {
    let text = Text::plain("Hello");
    let aligned = Align::center(text);

    let context = RenderContext { width: 20, height: None };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_align_left() {
    let text = Text::plain("Test");
    let aligned = Align::left(text);

    let context = RenderContext { width: 20, height: None };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_align_right() {
    let text = Text::plain("Right");
    let aligned = Align::right(text);

    let context = RenderContext { width: 20, height: None };
    let segments = aligned.render(&context);

    assert!(!segments.is_empty());
}

#[test]
fn test_vertical_alignment() {
    let text = Text::plain("Vertical");
    let aligned = Align::center(text)
        .vertical(VerticalAlignment::Middle)
        .height(5);

    let context = RenderContext { width: 20, height: None };
    let segments = aligned.render(&context);

    // Should have more segments due to vertical padding
    assert!(segments.len() >= 3); // empty lines + content + empty lines
}
