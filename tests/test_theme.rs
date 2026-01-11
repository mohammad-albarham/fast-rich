//! Integration tests for theme system

use fast_rich::style::Color;
use fast_rich::theme::Theme;

#[test]
fn test_default_theme() {
    let theme = Theme::default_theme();
    assert_eq!(theme.primary, Color::BrightBlue);
    assert_eq!(theme.success, Color::BrightGreen);
    assert_eq!(theme.error, Color::BrightRed);
}

#[test]
fn test_monokai_theme() {
    let theme = Theme::monokai();
    // Monokai uses RGB colors - just verify they exist
    let _ = theme.primary;
    let _ = theme.success;
    // Theme builds successfully (verified by compilation)
}

#[test]
fn test_night_owl_theme() {
    let theme = Theme::night_owl();
    let _ = theme.primary;
    let _ = theme.secondary;
    // Theme builds successfully (verified by compilation)
}

#[test]
fn test_get_style() {
    let theme = Theme::default_theme();
    let success_style = theme.get_style("success");
    // Style should be created successfully
    assert_eq!(success_style.foreground, Some(Color::BrightGreen));
}

#[test]
fn test_custom_colors() {
    let mut theme = Theme::new();
    theme.add_color("custom", Color::Magenta);

    let style = theme.get_style("custom");
    assert_eq!(style.foreground, Some(Color::Magenta));
}

#[test]
fn test_unknown_style() {
    let theme = Theme::new();
    let style = theme.get_style("nonexistent");
    assert_eq!(style.foreground, Some(Color::Default));
}

#[test]
fn test_theme_clone() {
    let theme1 = Theme::monokai();
    let theme2 = theme1.clone();

    assert_eq!(theme1.primary, theme2.primary);
    assert_eq!(theme1.success, theme2.success);
}
