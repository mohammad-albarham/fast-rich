//! Bidirectional text support (UAX #9).
//!
//! This module provides support for right-to-left (RTL) languages like Arabic
//! and Hebrew. It uses the Unicode Bidirectional Algorithm to reorder text
//! for visual display.
//!
//! # Example
//!
//! ```
//! use fast_rich::bidi::{TextDirection, reorder_for_display};
//!
//! // Arabic text mixed with English
//! let text = "مرحبا Hello";
//! let visual = reorder_for_display(text, TextDirection::Auto);
//! ```

/// Text direction options for RTL support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// Automatically detect direction from first strong character
    #[default]
    Auto,
    /// Force left-to-right direction
    Ltr,
    /// Force right-to-left direction
    Rtl,
}

#[cfg(feature = "rtl")]
use unicode_bidi::{BidiInfo, Level};

use crate::style::Style;
use crate::text::Span;
use std::borrow::Cow;

/// A styled range in logical text order (using character indices).
///
/// This tracks the character range and associated style for a segment of text,
/// allowing styles to be preserved during BiDi reordering.
#[derive(Debug, Clone)]
pub struct StyledRange {
    /// Start character index in the combined logical text
    pub start: usize,
    /// End character index in the combined logical text
    pub end: usize,
    /// The style associated with this range
    pub style: Style,
}

/// Reorder styled spans according to the Unicode Bidirectional Algorithm.
///
/// This is the core RTL algorithm that preserves the association between styles
/// and their logical text content while reordering for visual display.
///
/// # How it works
///
/// 1. Combines all span texts into one logical string
/// 2. Records which byte ranges have which styles
/// 3. Applies the Unicode BiDi algorithm to get visual ordering
/// 4. Reconstructs spans in visual order, preserving styles
///
/// # Example
///
/// ```ignore
/// use fast_rich::bidi::{reorder_styled_spans, TextDirection};
/// use fast_rich::text::Span;
/// use fast_rich::style::{Style, Color};
///
/// let red = Style::new().foreground(Color::Red);
/// let spans = vec![
///     Span::styled("مرحبا", red),  // Arabic "Hello" in red
///     Span::raw(" world"),
/// ];
///
/// let reordered = reorder_styled_spans(&spans, TextDirection::Rtl);
/// // The red style stays attached to the Arabic text
/// ```
///
/// # Note
/// This function performs both Arabic shaping (for letter connection) AND
/// BiDi reordering (for visual order). This is necessary because terminal
/// BiDi can interfere with ANSI escape codes.
#[cfg(feature = "rtl")]
pub fn reorder_styled_spans(spans: &[Span], direction: TextDirection) -> Vec<Span> {
    if spans.is_empty() {
        return Vec::new();
    }

    // 1. Reshape each span and build character->style mapping
    let mut logical_chars: Vec<char> = Vec::new();
    let mut char_styles: Vec<Style> = Vec::new();
    
    for span in spans {
        let reshaped = crate::shaping::reshape(&span.text);
        for c in reshaped.chars() {
            logical_chars.push(c);
            char_styles.push(span.style);
        }
    }
    
    if logical_chars.is_empty() {
        return Vec::new();
    }
    
    // 2. Build the combined logical text
    let logical_text: String = logical_chars.iter().collect();
    
    // 3. Run BiDi reordering to get visual text
    let paragraph_level = match direction {
        TextDirection::Ltr => Some(Level::ltr()),
        TextDirection::Rtl => Some(Level::rtl()),
        TextDirection::Auto => None,
    };
    
    let bidi_info = BidiInfo::new(&logical_text, paragraph_level);
    
    if bidi_info.paragraphs.is_empty() {
        // No paragraphs - return reshaped spans as-is
        return spans.iter().map(|span| {
            let reshaped = crate::shaping::reshape(&span.text);
            Span {
                text: Cow::Owned(reshaped.into_owned()),
                style: span.style,
                link: span.link.clone(),
            }
        }).collect();
    }
    
    // 4. Get visual text for each paragraph
    let mut visual_text = String::new();
    for para in &bidi_info.paragraphs {
        let line = para.range.clone();
        let reordered = bidi_info.reorder_line(para, line);
        visual_text.push_str(&reordered);
    }
    
    // 5. Map visual characters back to styles using character matching
    // We need to find which logical character each visual character came from
    let visual_chars: Vec<char> = visual_text.chars().collect();
    let mut visual_styles: Vec<Style> = Vec::with_capacity(visual_chars.len());
    
    // Track which logical characters have been used
    let mut used: Vec<bool> = vec![false; logical_chars.len()];
    
    for vc in &visual_chars {
        // Find the first unused matching character in logical order
        let mut found = false;
        for (i, (lc, used_flag)) in logical_chars.iter().zip(used.iter_mut()).enumerate() {
            if !*used_flag && lc == vc {
                visual_styles.push(char_styles[i]);
                *used_flag = true;
                found = true;
                break;
            }
        }
        if !found {
            // Fallback: use default style
            visual_styles.push(Style::default());
        }
    }
    
    // 6. Build result spans by merging adjacent same-styled characters
    let mut result: Vec<Span> = Vec::new();
    
    if visual_chars.is_empty() {
        return result;
    }
    
    let mut current_text = String::new();
    let mut current_style = visual_styles[0];
    
    for (c, style) in visual_chars.into_iter().zip(visual_styles) {
        if style != current_style {
            if !current_text.is_empty() {
                result.push(Span {
                    text: Cow::Owned(current_text),
                    style: current_style,
                    link: None,
                });
            }
            current_text = String::new();
            current_style = style;
        }
        current_text.push(c);
    }
    
    if !current_text.is_empty() {
        result.push(Span {
            text: Cow::Owned(current_text),
            style: current_style,
            link: None,
        });
    }
    
    result
}

/// Stub for when RTL feature is disabled
#[cfg(not(feature = "rtl"))]
pub fn reorder_styled_spans(spans: &[Span], _direction: TextDirection) -> Vec<Span> {
    spans.to_vec()
}

/// Check if a paragraph (text direction context) is RTL.
///
/// This uses the paragraph's embedding level to determine direction.
#[cfg(feature = "rtl")]
pub fn is_paragraph_rtl(text: &str, direction: TextDirection) -> bool {
    match direction {
        TextDirection::Rtl => true,
        TextDirection::Ltr => false,
        TextDirection::Auto => is_rtl(text),
    }
}

/// Stub for when RTL feature is disabled
#[cfg(not(feature = "rtl"))]
pub fn is_paragraph_rtl(_text: &str, _direction: TextDirection) -> bool {
    false
}

/// Reorder text for visual display according to the Unicode Bidirectional Algorithm.
///
/// This function takes logical text (stored in reading order) and returns
/// visual text (displayed order).
///
/// # Arguments
///
/// * `text` - The input text in logical order
/// * `direction` - The paragraph direction to use
///
/// # Example
///
/// ```
/// use fast_rich::bidi::{TextDirection, reorder_for_display};
///
/// // Arabic "مرحبا" mixed with "world"
/// let text = "مرحبا world";
/// let visual = reorder_for_display(text, TextDirection::Auto);
/// // In an RTL paragraph, English appears on the left
/// ```
#[cfg(feature = "rtl")]
pub fn reorder_for_display(text: &str, direction: TextDirection) -> String {
    if text.is_empty() {
        return String::new();
    }

    let paragraph_level = match direction {
        TextDirection::Ltr => Some(Level::ltr()),
        TextDirection::Rtl => Some(Level::rtl()),
        TextDirection::Auto => None,
    };

    // Reshape text 
    let shaped_cow = crate::shaping::reshape(text);
    let shaped_text = shaped_cow.as_ref();

    let bidi_info = BidiInfo::new(shaped_text, paragraph_level);

    if bidi_info.paragraphs.is_empty() {
        return text.to_string();
    }

    // Handle multi-paragraph text
    let mut result = String::with_capacity(text.len());
    for para in &bidi_info.paragraphs {
        let line = para.range.clone();
        let reordered = bidi_info.reorder_line(para, line);
        result.push_str(&reordered);
    }

    result
}

/// Stub for when RTL feature is disabled
#[cfg(not(feature = "rtl"))]
pub fn reorder_for_display(text: &str, _direction: TextDirection) -> String {
    text.to_string()
}

/// Get the base direction of text (auto-detected).
///
/// Returns `true` if the text is primarily RTL.
#[cfg(feature = "rtl")]
pub fn is_rtl(text: &str) -> bool {
    use unicode_bidi::get_base_direction;
    use unicode_bidi::Direction;

    matches!(get_base_direction(text), Direction::Rtl)
}

/// Stub for when RTL feature is disabled
#[cfg(not(feature = "rtl"))]
pub fn is_rtl(_text: &str) -> bool {
    false
}

/// Mirror paired characters for RTL context.
///
/// Swaps brackets, parentheses, and other directional characters.
pub fn mirror_char(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        '«' => '»',
        '»' => '«',
        '‹' => '›',
        '›' => '‹',
        '⟨' => '⟩',
        '⟩' => '⟨',
        '⟪' => '⟫',
        '⟫' => '⟪',
        _ => c,
    }
}

/// Mirror all paired characters in a string.
pub fn mirror_string(text: &str) -> String {
    text.chars().map(mirror_char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_direction_default() {
        assert_eq!(TextDirection::default(), TextDirection::Auto);
    }

    #[test]
    fn test_mirror_brackets() {
        assert_eq!(mirror_char('('), ')');
        assert_eq!(mirror_char(')'), '(');
        assert_eq!(mirror_char('['), ']');
        assert_eq!(mirror_char(']'), '[');
        assert_eq!(mirror_char('{'), '}');
        assert_eq!(mirror_char('}'), '{');
        assert_eq!(mirror_char('<'), '>');
        assert_eq!(mirror_char('>'), '<');
    }

    #[test]
    fn test_mirror_guillemets() {
        assert_eq!(mirror_char('«'), '»');
        assert_eq!(mirror_char('»'), '«');
    }

    #[test]
    fn test_mirror_unchanged() {
        assert_eq!(mirror_char('a'), 'a');
        assert_eq!(mirror_char('م'), 'م'); // Arabic letter
        assert_eq!(mirror_char('1'), '1');
    }

    #[test]
    fn test_mirror_string() {
        assert_eq!(mirror_string("(hello)"), ")hello(");
        assert_eq!(mirror_string("[a{b}c]"), "]a}b{c[");
    }

    #[cfg(feature = "rtl")]
    mod rtl_tests {
        use super::*;

        #[test]
        fn test_arabic_only() {
            // Pure Arabic text - should remain unchanged (already in visual order)
            let text = "مرحبا";
            let visual = reorder_for_display(text, TextDirection::Auto);
            // Note: Pure RTL text stays the same, reordering happens with mixed text
            assert!(!visual.is_empty());
        }

        #[test]
        fn test_arabic_with_english() {
            // Arabic "مرحبا" + English "Hello"
            let text = "مرحبا Hello";
            let visual = reorder_for_display(text, TextDirection::Auto);
            
            // BiDi reordering reorders characters for visual display
            // The result should contain both the Arabic chars and "Hello"
            assert!(visual.contains("Hello"), "Visual should contain 'Hello': {}", visual);
            
            // For Arabic, the characters are now SHAPED (Presentation Forms B).
            // So we reshape the reference string to check against.
            let shaped_ref = crate::shaping::reshape("مرحبا");
            
            // Check that Arabic characters are present (may be reordered individually)
            assert!(visual.chars().any(|c| shaped_ref.as_ref().contains(c)), "Visual should contain shaped Arabic chars from {}: {}", shaped_ref, visual);
            
            // Total length should be preserved
            assert_eq!(visual.chars().count(), text.chars().count(), "Character count should match");
        }

        #[test]
        fn test_arabic_sentence() {
            // "أهلا وسهلا" = "Welcome" in Arabic
            let text = "أهلا وسهلا";
            let visual = reorder_for_display(text, TextDirection::Rtl);
            assert!(!visual.is_empty());
        }

        #[test]
        fn test_english_only_ltr() {
            let text = "Hello World";
            let visual = reorder_for_display(text, TextDirection::Ltr);
            assert_eq!(visual, "Hello World");
        }

        #[test]
        fn test_forced_rtl_direction() {
            let text = "ABC";
            let visual = reorder_for_display(text, TextDirection::Rtl);
            // Even LTR text in RTL context stays left-to-right internally
            assert_eq!(visual, "ABC");
        }

        #[test]
        fn test_empty_string() {
            let visual = reorder_for_display("", TextDirection::Auto);
            assert_eq!(visual, "");
        }

        #[test]
        fn test_is_rtl_arabic() {
            assert!(is_rtl("مرحبا"));
            assert!(is_rtl("أهلا وسهلا"));
        }

        #[test]
        fn test_is_rtl_english() {
            assert!(!is_rtl("Hello"));
            assert!(!is_rtl("Hello World"));
        }

        #[test]
        fn test_is_rtl_mixed_arabic_first() {
            // Arabic first = RTL
            assert!(is_rtl("مرحبا Hello"));
        }

        #[test]
        fn test_is_rtl_mixed_english_first() {
            // English first = LTR
            assert!(!is_rtl("Hello مرحبا"));
        }

        #[test]
        fn test_arabic_numbers() {
            // Numbers in Arabic context
            let text = "السعر 100 دولار";  // "The price is 100 dollars"
            let visual = reorder_for_display(text, TextDirection::Auto);
            assert!(visual.contains("100"));
        }

        #[test]
        fn test_arabic_with_punctuation() {
            // Arabic with punctuation
            let text = "مرحبا!";  // "Hello!"
            let visual = reorder_for_display(text, TextDirection::Auto);
            assert!(visual.contains("!"));
        }

        #[test]
        fn test_reorder_styled_spans_basic() {
            use crate::text::Span;
            use crate::style::{Style, Color};

            let red = Style::new().foreground(Color::Red);
            let spans = vec![
                Span::styled("Hello", red),
                Span::raw(" World"),
            ];

            // LTR text in LTR direction -> no change
            let reordered = reorder_styled_spans(&spans, TextDirection::Ltr);
            assert_eq!(reordered.len(), 2);
            assert_eq!(reordered[0].text, "Hello");
            assert_eq!(reordered[1].text, " World");
        }

        #[test]
        fn test_reorder_styled_spans_mixed_rtl() {
            use crate::text::Span;
            use crate::style::{Style, Color};

            let red = Style::new().foreground(Color::Red);
            let green = Style::new().foreground(Color::Green);
            // "Hello " (Red) + "مرحبا" (Green)
            // In RTL context
            let spans = vec![
                Span::styled("Hello ", red),
                Span::styled("مرحبا", green),
            ];

            let reordered = reorder_styled_spans(&spans, TextDirection::Rtl);
            
            // visual text should have Arabic reversed if reordering happened
            // So we can't check for "مرحبا" directly.
            // Check that we have spans with correct styles
            let has_red_hello = reordered.iter().any(|s| s.text.contains("Hello") && s.style.foreground == Some(Color::Red));
            
            // For Arabic, check that we have a Green span with Arabic chars
            let shaped_ref = crate::shaping::reshape("مرحبا");
            let has_green_arabic = reordered.iter().any(|s| 
                s.style.foreground == Some(Color::Green) && 
                s.text.chars().any(|c| shaped_ref.contains(c))
            );
            
            assert!(has_red_hello, "Should contain Red Hello");
            assert!(has_green_arabic, "Should contain Green Arabic characters");
        }

    }
}
