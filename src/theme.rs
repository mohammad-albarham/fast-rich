//! Theme system for consistent styling.
//!
//! Themes provide predefined color palettes and style configurations.

use crate::style::{Color, Style};
use std::collections::HashMap;

/// A theme defines colors and styles for different semantic elements.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Primary color
    pub primary: Color,
    /// Secondary color
    pub secondary: Color,
    /// Success/positive color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error/danger color
    pub error: Color,
    /// Info color
    pub info: Color,
    /// Muted/dim color
    pub muted: Color,
    /// Custom named colors
    pub custom: HashMap<String, Color>,
    /// Named styles (like Rich's `Theme({"warning": "bold red"})`), used by
    /// markup tags such as `[warning]`. Takes precedence over colors.
    pub styles: HashMap<String, Style>,
}

impl Theme {
    /// Create a new empty theme with defaults.
    pub fn new() -> Self {
        Theme {
            primary: Color::Blue,
            secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::BrightBlack,
            custom: HashMap::new(),
            styles: HashMap::new(),
        }
    }

    /// Get the default theme (similar to Rich's default).
    pub fn default_theme() -> Self {
        let mut theme = Theme::new();
        theme.primary = Color::BrightBlue;
        theme.secondary = Color::Magenta;
        theme.success = Color::BrightGreen;
        theme.warning = Color::BrightYellow;
        theme.error = Color::BrightRed;
        theme.info = Color::BrightCyan;
        theme.muted = Color::BrightBlack;
        theme
    }

    /// Monokai-inspired theme.
    pub fn monokai() -> Self {
        Theme {
            primary: Color::rgb(102, 217, 239),  // Cyan
            secondary: Color::rgb(249, 38, 114), // Pink
            success: Color::rgb(166, 226, 46),   // Green
            warning: Color::rgb(253, 151, 31),   // Orange
            error: Color::rgb(249, 38, 114),     // Pink
            info: Color::rgb(174, 129, 255),     // Purple
            muted: Color::rgb(117, 113, 94),     // Gray
            custom: HashMap::new(),
            styles: HashMap::new(),
        }
    }

    /// Night Owl theme.
    pub fn night_owl() -> Self {
        Theme {
            primary: Color::rgb(130, 170, 255),   // Blue
            secondary: Color::rgb(199, 146, 234), // Purple
            success: Color::rgb(173, 219, 103),   // Green
            warning: Color::rgb(255, 203, 107),   // Yellow
            error: Color::rgb(239, 83, 80),       // Red
            info: Color::rgb(128, 203, 196),      // Teal
            muted: Color::rgb(99, 119, 119),      // Gray
            custom: HashMap::new(),
            styles: HashMap::new(),
        }
    }

    /// Get a style for a semantic element.
    pub fn get_style(&self, name: &str) -> Style {
        let color = match name {
            "primary" => self.primary,
            "secondary" => self.secondary,
            "success" => self.success,
            "warning" => self.warning,
            "error" => self.error,
            "info" => self.info,
            "muted" => self.muted,
            _ => self.custom.get(name).copied().unwrap_or(Color::Default),
        };
        Style::new().foreground(color)
    }

    /// Look up a named style: [`styles`](Self::styles) first, then the
    /// semantic colors (`primary`, `warning`, ...) and custom colors.
    /// Returns `None` for names the theme doesn't define.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::theme::Theme;
    /// let mut theme = Theme::new();
    /// theme.add_style("alert", Style::parse("bold red"));
    /// assert!(theme.lookup_style("alert").unwrap().bold);
    /// assert!(theme.lookup_style("warning").is_some());
    /// assert!(theme.lookup_style("nope").is_none());
    /// ```
    pub fn lookup_style(&self, name: &str) -> Option<Style> {
        if let Some(style) = self.styles.get(name) {
            return Some(*style);
        }
        let color = match name {
            "primary" => self.primary,
            "secondary" => self.secondary,
            "success" => self.success,
            "warning" => self.warning,
            "error" => self.error,
            "info" => self.info,
            "muted" => self.muted,
            _ => *self.custom.get(name)?,
        };
        Some(Style::new().foreground(color))
    }

    /// Add a named style to the theme (used by markup like `[name]`).
    pub fn add_style(&mut self, name: impl Into<String>, style: Style) {
        self.styles.insert(name.into(), style);
    }

    /// Add a custom color to the theme.
    pub fn add_color(&mut self, name: impl Into<String>, color: Color) {
        self.custom.insert(name.into(), color);
    }
}

impl Default for Theme {
    /// `default` and `new` return the same palette; use
    /// [`Theme::default_theme`] for the Rich-like bright variant.
    fn default() -> Self {
        Theme::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default_theme_are_consistent() {
        // `new` and `default` must agree — they historically returned
        // different palettes (dim vs bright variants), a footgun.
        assert_eq!(Theme::new().primary, Theme::default().primary);
    }

    #[test]
    fn test_monokai_differs_from_default() {
        assert_ne!(Theme::monokai().primary, Theme::default().primary);
    }

    #[test]
    fn test_custom_color_lookup() {
        let mut theme = Theme::new();
        theme
            .custom
            .insert("brand".to_string(), Color::rgb(1, 2, 3));
        let style = theme.get_style("brand");
        assert_eq!(style.foreground, Some(Color::rgb(1, 2, 3)));
    }

    #[test]
    fn test_unknown_style_name_falls_back() {
        let theme = Theme::new();
        let _ = theme.get_style("does-not-exist");
    }
}
