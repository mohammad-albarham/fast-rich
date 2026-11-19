//! Theme system for consistent styling.
//!
//! Themes provide predefined color palettes and style configurations.

use crate::style::{Color, Style};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

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
            _ => match self.custom.get(name) {
                Some(color) => *color,
                None => return builtin_style(name),
            },
        };
        Some(Style::new().foreground(color))
    }

    /// Resolve a named style, or an empty style for unknown names (Rich's
    /// `Console.get_style` without the parse fallback). Theme styles win
    /// over Rich's defaults ([`builtin_style`]).
    ///
    /// ```
    /// use fast_rich::style::{Color, Style};
    /// use fast_rich::theme::Theme;
    /// let theme = Theme::new().with_style("table.header", Style::parse("bold magenta"));
    /// assert_eq!(theme.style("table.header").foreground, Some(Color::Magenta));
    /// assert!(theme.style("table.title").italic); // Rich's default
    /// assert!(theme.style("unknown").is_empty());
    /// ```
    pub fn style(&self, name: &str) -> Style {
        self.lookup_style(name).unwrap_or_default()
    }

    /// Add a named style to the theme (used by markup like `[name]` and by
    /// components such as `table.header`). Overrides Rich's default for
    /// that name.
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::theme::Theme;
    /// let mut theme = Theme::new();
    /// theme.add_style("rule.line", Style::parse("red"));
    /// assert!(theme.style("rule.line").foreground.is_some());
    /// ```
    pub fn add_style(&mut self, name: impl Into<String>, style: Style) {
        self.styles.insert(name.into(), style);
    }

    /// Builder form of [`add_style`](Self::add_style).
    ///
    /// ```
    /// use fast_rich::style::Style;
    /// use fast_rich::theme::Theme;
    /// let theme = Theme::new().with_style("log.time", Style::parse("bold"));
    /// assert!(theme.style("log.time").bold);
    /// ```
    pub fn with_style(mut self, name: impl Into<String>, style: Style) -> Self {
        self.add_style(name, style);
        self
    }

    /// Build a theme from `(name, style string)` pairs, like Rich's
    /// `Theme({"table.header": "bold magenta"})`. Names not given keep
    /// Rich's defaults.
    ///
    /// ```
    /// use fast_rich::style::Color;
    /// use fast_rich::theme::Theme;
    /// let theme = Theme::from_styles([("table.header", "bold magenta"), ("bar.complete", "cyan")]);
    /// assert_eq!(theme.style("bar.complete").foreground, Some(Color::Cyan));
    /// ```
    pub fn from_styles<I, K, V>(styles: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: AsRef<str>,
    {
        let mut theme = Theme::new();
        for (name, style) in styles {
            theme.add_style(name, Style::parse(style.as_ref()));
        }
        theme
    }

    /// Add a custom color to the theme.
    pub fn add_color(&mut self, name: impl Into<String>, color: Color) {
        self.custom.insert(name.into(), color);
    }
}

/// Rich's built-in named styles that every theme falls back to.
///
/// All 154 names of Rich's `DEFAULT_STYLES` (`rich/default_styles.py`):
/// `table.header`, `bar.complete`, `log.time`, `repr.number`,
/// `markdown.h1`, ... with Rich's values. Components look their styles up by
/// these names through the console theme, so any of them can be restyled
/// with [`Theme::add_style`]. Returns `None` for names Rich doesn't define.
///
/// Rich's `not bold` (attribute explicitly off) becomes "bold not set",
/// since [`Style`] has no separate "unset" state; `blink2`, `underline2`,
/// `frame`, `encircle` and `overline` are dropped (no `Style` field).
///
/// ```
/// use fast_rich::style::Color;
/// use fast_rich::theme::builtin_style;
/// assert!(builtin_style("table.header").unwrap().bold);
/// assert_eq!(builtin_style("bar.back").unwrap().foreground, Some(Color::Ansi256(237)));
/// assert!(builtin_style("no.such.style").is_none());
/// ```
pub fn builtin_style(name: &str) -> Option<Style> {
    static STYLES: OnceLock<HashMap<&'static str, Style>> = OnceLock::new();
    STYLES
        .get_or_init(|| {
            crate::default_styles::DEFAULT_STYLES
                .iter()
                .map(|(name, style)| (*name, Style::parse(style)))
                .collect()
        })
        .get(name)
        .copied()
}

/// The names of Rich's default styles, in Rich's order (see
/// [`builtin_style`]).
///
/// ```
/// let names: Vec<&str> = fast_rich::theme::builtin_style_names().collect();
/// assert_eq!(names.len(), 154);
/// assert!(names.contains(&"progress.percentage"));
/// ```
pub fn builtin_style_names() -> impl Iterator<Item = &'static str> {
    crate::default_styles::DEFAULT_STYLES
        .iter()
        .map(|(name, _)| *name)
}

/// A shared, empty [`Theme::new`] (Rich's defaults only), so default
/// render contexts don't allocate a theme each time.
pub(crate) fn shared_default() -> Arc<Theme> {
    static DEFAULT: OnceLock<Arc<Theme>> = OnceLock::new();
    DEFAULT.get_or_init(|| Arc::new(Theme::new())).clone()
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
