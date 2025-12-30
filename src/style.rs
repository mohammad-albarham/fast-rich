//! Style and color types for terminal output.
//!
//! This module provides the core styling primitives used throughout rich-rust.
//!
//! # Examples
//!
//! ```
//! use rich_rust::style::{Color, Style};
//!
//! let style = Style::new()
//!     .foreground(Color::Red)
//!     .bold()
//!     .underline();
//! ```

use std::fmt;

/// Represents a terminal color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Default terminal color
    Default,
    /// Black color
    Black,
    /// Red color
    Red,
    /// Green color
    Green,
    /// Yellow color
    Yellow,
    /// Blue color
    Blue,
    /// Magenta color
    Magenta,
    /// Cyan color
    Cyan,
    /// White color
    White,
    /// Bright black (gray)
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
    /// 256-color palette (0-255)
    Ansi256(u8),
    /// True color RGB
    Rgb { r: u8, g: u8, b: u8 },
}

impl Color {
    /// Create a new RGB color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb { r, g, b }
    }

    /// Create a new 256-color palette color.
    pub fn ansi256(code: u8) -> Self {
        Color::Ansi256(code)
    }

    /// Parse a color from a string.
    ///
    /// Supports:
    /// - Named colors: "red", "blue", "bright_red", etc.
    /// - Hex colors: "#ff0000", "#f00"
    /// - RGB: "rgb(255, 0, 0)"
    /// - 256-color: "color(196)"
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase();

        // Named colors
        match s.as_str() {
            "default" => return Some(Color::Default),
            "black" => return Some(Color::Black),
            "red" => return Some(Color::Red),
            "green" => return Some(Color::Green),
            "yellow" => return Some(Color::Yellow),
            "blue" => return Some(Color::Blue),
            "magenta" => return Some(Color::Magenta),
            "cyan" => return Some(Color::Cyan),
            "white" => return Some(Color::White),
            "bright_black" | "brightblack" | "grey" | "gray" => return Some(Color::BrightBlack),
            "bright_red" | "brightred" => return Some(Color::BrightRed),
            "bright_green" | "brightgreen" => return Some(Color::BrightGreen),
            "bright_yellow" | "brightyellow" => return Some(Color::BrightYellow),
            "bright_blue" | "brightblue" => return Some(Color::BrightBlue),
            "bright_magenta" | "brightmagenta" => return Some(Color::BrightMagenta),
            "bright_cyan" | "brightcyan" => return Some(Color::BrightCyan),
            "bright_white" | "brightwhite" => return Some(Color::BrightWhite),
            _ => {}
        }

        // Hex colors: #rgb or #rrggbb
        if let Some(hex) = s.strip_prefix('#') {
            return Self::parse_hex(hex);
        }

        // RGB: rgb(r, g, b)
        if let Some(inner) = s.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                let r = parts[0].trim().parse().ok()?;
                let g = parts[1].trim().parse().ok()?;
                let b = parts[2].trim().parse().ok()?;
                return Some(Color::Rgb { r, g, b });
            }
        }

        // 256-color: color(n)
        if let Some(inner) = s.strip_prefix("color(").and_then(|s| s.strip_suffix(')')) {
            let code: u8 = inner.trim().parse().ok()?;
            return Some(Color::Ansi256(code));
        }

        None
    }

    fn parse_hex(hex: &str) -> Option<Self> {
        match hex.len() {
            3 => {
                // #rgb -> #rrggbb
                let mut chars = hex.chars();
                let r = chars.next()?;
                let g = chars.next()?;
                let b = chars.next()?;
                let r = u8::from_str_radix(&format!("{r}{r}"), 16).ok()?;
                let g = u8::from_str_radix(&format!("{g}{g}"), 16).ok()?;
                let b = u8::from_str_radix(&format!("{b}{b}"), 16).ok()?;
                Some(Color::Rgb { r, g, b })
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Color::Rgb { r, g, b })
            }
            _ => None,
        }
    }

    /// Convert to crossterm color.
    pub fn to_crossterm(&self) -> crossterm::style::Color {
        match self {
            Color::Default => crossterm::style::Color::Reset,
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::DarkRed,
            Color::Green => crossterm::style::Color::DarkGreen,
            Color::Yellow => crossterm::style::Color::DarkYellow,
            Color::Blue => crossterm::style::Color::DarkBlue,
            Color::Magenta => crossterm::style::Color::DarkMagenta,
            Color::Cyan => crossterm::style::Color::DarkCyan,
            Color::White => crossterm::style::Color::Grey,
            Color::BrightBlack => crossterm::style::Color::DarkGrey,
            Color::BrightRed => crossterm::style::Color::Red,
            Color::BrightGreen => crossterm::style::Color::Green,
            Color::BrightYellow => crossterm::style::Color::Yellow,
            Color::BrightBlue => crossterm::style::Color::Blue,
            Color::BrightMagenta => crossterm::style::Color::Magenta,
            Color::BrightCyan => crossterm::style::Color::Cyan,
            Color::BrightWhite => crossterm::style::Color::White,
            Color::Ansi256(code) => crossterm::style::Color::AnsiValue(*code),
            Color::Rgb { r, g, b } => crossterm::style::Color::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
        }
    }

    /// Convert color to CSS color string.
    pub fn to_css(&self) -> String {
        match self {
            Color::Default => "inherit".to_string(),
            Color::Black => "#000000".to_string(),
            Color::Red => "#cd0000".to_string(),
            Color::Green => "#00cd00".to_string(),
            Color::Yellow => "#cdcd00".to_string(),
            Color::Blue => "#0000cd".to_string(),
            Color::Magenta => "#cd00cd".to_string(),
            Color::Cyan => "#00cdcd".to_string(),
            Color::White => "#e5e5e5".to_string(),
            Color::BrightBlack => "#7f7f7f".to_string(),
            Color::BrightRed => "#ff0000".to_string(),
            Color::BrightGreen => "#00ff00".to_string(),
            Color::BrightYellow => "#ffff00".to_string(),
            Color::BrightBlue => "#5c5cff".to_string(),
            Color::BrightMagenta => "#ff00ff".to_string(),
            Color::BrightCyan => "#00ffff".to_string(),
            Color::BrightWhite => "#ffffff".to_string(),
            Color::Ansi256(code) => format!("var(--ansi-{})", code),
            Color::Rgb { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

/// Style attributes for text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Style {
    /// Foreground color
    pub foreground: Option<Color>,
    /// Background color
    pub background: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Dim/faint text
    pub dim: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Blinking text
    pub blink: bool,
    /// Reversed colors (fg/bg swapped)
    pub reverse: bool,
    /// Hidden/invisible text
    pub hidden: bool,
    /// Strikethrough text
    pub strikethrough: bool,
}

impl Style {
    /// Create a new empty style.
    pub const fn new() -> Self {
        Style {
            foreground: None,
            background: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            hidden: false,
            strikethrough: false,
        }
    }

    /// Set the foreground color.
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set the foreground color (alias for consistency with Rich).
    pub fn fg(self, color: Color) -> Self {
        self.foreground(color)
    }

    /// Set the background color (alias for consistency with Rich).
    pub fn bg(self, color: Color) -> Self {
        self.background(color)
    }

    /// Enable bold.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enable dim/faint.
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Enable italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enable underline.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Enable blink.
    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    /// Enable reverse (swap fg/bg).
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Enable hidden/invisible.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Enable strikethrough.
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Combine this style with another, with `other` taking precedence.
    pub fn combine(&self, other: &Style) -> Style {
        Style {
            foreground: other.foreground.or(self.foreground),
            background: other.background.or(self.background),
            bold: self.bold || other.bold,
            dim: self.dim || other.dim,
            italic: self.italic || other.italic,
            underline: self.underline || other.underline,
            blink: self.blink || other.blink,
            reverse: self.reverse || other.reverse,
            hidden: self.hidden || other.hidden,
            strikethrough: self.strikethrough || other.strikethrough,
        }
    }

    /// Check if this style has any attributes set.
    pub fn is_empty(&self) -> bool {
        self.foreground.is_none()
            && self.background.is_none()
            && !self.bold
            && !self.dim
            && !self.italic
            && !self.underline
            && !self.blink
            && !self.reverse
            && !self.hidden
            && !self.strikethrough
    }

    /// Parse a style from a string.
    ///
    /// Supports space-separated attributes: "bold red on blue"
    pub fn parse(s: &str) -> Self {
        let mut style = Style::new();
        let mut on_background = false;

        for part in s.split_whitespace() {
            let part_lower = part.to_lowercase();

            if part_lower == "on" {
                on_background = true;
                continue;
            }

            // Check for attributes
            match part_lower.as_str() {
                "bold" | "b" => style.bold = true,
                "dim" => style.dim = true,
                "italic" | "i" => style.italic = true,
                "underline" | "u" => style.underline = true,
                "blink" => style.blink = true,
                "reverse" => style.reverse = true,
                "hidden" => style.hidden = true,
                "strike" | "strikethrough" | "s" => style.strikethrough = true,
                "not" => {
                    // "not bold" etc. - skip for now, just consume
                    continue;
                }
                _ => {
                    // Try to parse as color
                    if let Some(color) = Color::parse(&part_lower) {
                        if on_background {
                            style.background = Some(color);
                            on_background = false;
                        } else {
                            style.foreground = Some(color);
                        }
                    }
                }
            }
        }

        style
    }

    /// Apply this style to crossterm for rendering.
    pub fn to_crossterm_attributes(&self) -> crossterm::style::Attributes {
        use crossterm::style::Attribute;
        let mut attrs = crossterm::style::Attributes::default();

        if self.bold {
            attrs.set(Attribute::Bold);
        }
        if self.dim {
            attrs.set(Attribute::Dim);
        }
        if self.italic {
            attrs.set(Attribute::Italic);
        }
        if self.underline {
            attrs.set(Attribute::Underlined);
        }
        if self.blink {
            attrs.set(Attribute::SlowBlink);
        }
        if self.reverse {
            attrs.set(Attribute::Reverse);
        }
        if self.hidden {
            attrs.set(Attribute::Hidden);
        }
        if self.strikethrough {
            attrs.set(Attribute::CrossedOut);
        }

        attrs
    }

    /// Convert this style to CSS inline style string.
    ///
    /// Returns a string suitable for use in HTML style attributes.
    pub fn to_css(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref fg) = self.foreground {
            parts.push(format!("color: {}", fg.to_css()));
        }
        if let Some(ref bg) = self.background {
            parts.push(format!("background-color: {}", bg.to_css()));
        }
        if self.bold {
            parts.push("font-weight: bold".to_string());
        }
        if self.italic {
            parts.push("font-style: italic".to_string());
        }
        if self.underline {
            parts.push("text-decoration: underline".to_string());
        }
        if self.strikethrough {
            parts.push("text-decoration: line-through".to_string());
        }
        if self.dim {
            parts.push("opacity: 0.5".to_string());
        }

        parts.join("; ")
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.bold {
            parts.push("bold");
        }
        if self.dim {
            parts.push("dim");
        }
        if self.italic {
            parts.push("italic");
        }
        if self.underline {
            parts.push("underline");
        }
        if self.strikethrough {
            parts.push("strikethrough");
        }

        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parse_named() {
        assert_eq!(Color::parse("red"), Some(Color::Red));
        assert_eq!(Color::parse("Blue"), Some(Color::Blue));
        assert_eq!(Color::parse("BRIGHT_RED"), Some(Color::BrightRed));
        assert_eq!(Color::parse("grey"), Some(Color::BrightBlack));
    }

    #[test]
    fn test_color_parse_hex() {
        assert_eq!(
            Color::parse("#ff0000"),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("#f00"),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Color::parse("#abc"),
            Some(Color::Rgb {
                r: 170,
                g: 187,
                b: 204
            })
        );
    }

    #[test]
    fn test_color_parse_rgb() {
        assert_eq!(
            Color::parse("rgb(255, 128, 64)"),
            Some(Color::Rgb {
                r: 255,
                g: 128,
                b: 64
            })
        );
    }

    #[test]
    fn test_color_parse_ansi256() {
        assert_eq!(Color::parse("color(196)"), Some(Color::Ansi256(196)));
    }

    #[test]
    fn test_style_parse() {
        let style = Style::parse("bold red on blue");
        assert!(style.bold);
        assert_eq!(style.foreground, Some(Color::Red));
        assert_eq!(style.background, Some(Color::Blue));
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new().foreground(Color::Green).bold().underline();

        assert!(style.bold);
        assert!(style.underline);
        assert_eq!(style.foreground, Some(Color::Green));
        assert!(!style.italic);
    }

    #[test]
    fn test_style_combine() {
        let base = Style::new().foreground(Color::Red).bold();
        let overlay = Style::new().foreground(Color::Blue).italic();
        let combined = base.combine(&overlay);

        assert_eq!(combined.foreground, Some(Color::Blue)); // overlay wins
        assert!(combined.bold); // kept from base
        assert!(combined.italic); // added from overlay
    }

    #[test]
    fn test_style_is_empty() {
        assert!(Style::new().is_empty());
        assert!(!Style::new().bold().is_empty());
    }
}
