//! Integration tests for padding wrapper

use rich_rust::padding::{Padding, PaddingSpec};
use rich_rust::text::Text;
use rich_rust::console::RenderContext;
use rich_rust::renderable::Renderable;

#[test]
fn test_padding_all() {
    let text = Text::plain("Content");
    let padded = Padding::all(text, 2);
    
    let context = RenderContext { width: 20 };
    let segments = padded.render(&context);
    
    // Should have top padding + content + bottom padding
    assert!(segments.len() >= 3);
}

#[test]
fn test_padding_symmetric() {
    let text = Text::plain("Test");
    let padded = Padding::symmetric(text, 1, 3); // 1 vertical, 3 horizontal
    
    let context = RenderContext { width: 20 };
    let segments = padded.render(&context);
    
    assert!(!segments.is_empty());
}

#[test]
fn test_padding_spec() {
    let spec = PaddingSpec::new(1, 2, 3, 4);
    assert_eq!(spec.top, 1);
    assert_eq!(spec.right, 2);
    assert_eq!(spec.bottom, 3);
    assert_eq!(spec.left, 4);
}

#[test]
fn test_padding_spec_all() {
    let spec = PaddingSpec::all(5);
    assert_eq!(spec.top, 5);
    assert_eq!(spec.right, 5);
    assert_eq!(spec.bottom, 5);
    assert_eq!(spec.left, 5);
}

#[test]
fn test_padding_spec_symmetric() {
    let spec = PaddingSpec::symmetric(2, 4);
    assert_eq!(spec.top, 2);
    assert_eq!(spec.bottom, 2);
    assert_eq!(spec.left, 4);
    assert_eq!(spec.right, 4);
}

#[test]
fn test_padding_none() {
    let spec = PaddingSpec::none();
    assert_eq!(spec.top, 0);
    assert_eq!(spec.right, 0);
    assert_eq!(spec.bottom, 0);
    assert_eq!(spec.left, 0);
}
