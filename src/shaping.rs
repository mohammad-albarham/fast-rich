//! Arabic text shaping module.
//!
//! Uses the `ar-reshaper` crate to convert logical Arabic characters into their
//! contextual Presentation Forms. This ensures proper cursive connection in
//! terminals that don't natively support Arabic script shaping.
//!
//! The `ar-reshaper` crate is a mature port of Python's `python-arabic-reshaper`
//! and handles all edge cases for Arabic, Persian, and Urdu scripts.

use std::borrow::Cow;

#[cfg(feature = "rtl")]
use ar_reshaper::ArabicReshaper;

#[cfg(feature = "rtl")]
use std::sync::LazyLock;

/// Lazily initialized reshaper instance (thread-safe singleton)
#[cfg(feature = "rtl")]
static RESHAPER: LazyLock<ArabicReshaper> = LazyLock::new(|| ArabicReshaper::default());

/// Reshape Arabic text for proper cursive display.
///
/// This function converts Arabic characters to their Presentation Forms B
/// (Isolated, Initial, Medial, Final) based on context, ensuring letters
/// connect properly when displayed in terminal environments.
///
/// # Arguments
/// * `text` - Input text potentially containing Arabic characters
///
/// # Returns
/// * `Cow::Borrowed(text)` if no reshaping needed
/// * `Cow::Owned(reshaped)` if reshaping was applied
///
/// # Example
/// ```
/// use fast_rich::shaping::reshape;
///
/// let text = "السلام عليكم";
/// let reshaped = reshape(text);
/// // Returns properly connected Arabic letters
/// ```
#[cfg(feature = "rtl")]
pub fn reshape(text: &str) -> Cow<'_, str> {
    // Fast path: check if reshaping is needed
    if !RESHAPER.need_reshape(text) {
        return Cow::Borrowed(text);
    }
    
    Cow::Owned(RESHAPER.reshape(text))
}

/// Stub implementation when RTL feature is disabled
#[cfg(not(feature = "rtl"))]
pub fn reshape(text: &str) -> Cow<'_, str> {
    Cow::Borrowed(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "rtl")]
    fn test_reshape_arabic() {
        // Test basic Arabic reshaping
        let input = "السلام عليكم";
        let reshaped = reshape(input);
        
        // The reshaped text should be different (Presentation Forms B)
        assert_ne!(input, reshaped.as_ref());
        
        // Should contain connected forms
        assert!(reshaped.chars().any(|c| {
            let u = c as u32;
            // Presentation Forms B range
            (0xFE70..=0xFEFF).contains(&u)
        }));
    }

    #[test]
    #[cfg(feature = "rtl")]
    fn test_reshape_preserves_english() {
        let input = "Hello World";
        let reshaped = reshape(input);
        
        // English text should not be modified
        assert_eq!(input, reshaped.as_ref());
    }

    #[test]
    #[cfg(feature = "rtl")]
    fn test_reshape_mixed_text() {
        let input = "Hello مرحبا World";
        let reshaped = reshape(input);
        
        // Should contain both English (unchanged) and reshaped Arabic
        assert!(reshaped.contains("Hello"));
        assert!(reshaped.contains("World"));
    }
}
