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

    /// Add a custom color to the theme.
    pub fn add_color(&mut self, name: impl Into<String>, color: Color) {
        self.custom.insert(name.into(), color);
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::default_theme()
    }
}
