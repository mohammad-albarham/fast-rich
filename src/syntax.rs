//! Syntax highlighting for source code.
//!
//! This module provides syntax highlighting using syntect.
//! It's feature-gated behind the `syntax` feature.

use crate::console::RenderContext;
use crate::panel::{BorderStyle, Panel};
use crate::renderable::{Renderable, Segment};
use crate::style::{Color, Style};
use crate::text::{Span, Text};
use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style as SyntectStyle, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

/// Syntax highlighting theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Monokai (dark theme)
    #[default]
    Monokai,
    /// InspiredGitHub (light theme)
    InspiredGitHub,
    /// Solarized Dark
    SolarizedDark,
    /// Solarized Light
    SolarizedLight,
    /// Base16 Ocean Dark
    Base16OceanDark,
}

impl Theme {
    fn name(&self) -> &'static str {
        match self {
            Theme::Monokai => "base16-monokai.dark",
            Theme::InspiredGitHub => "InspiredGitHub",
            Theme::SolarizedDark => "Solarized (dark)",
            Theme::SolarizedLight => "Solarized (light)",
            Theme::Base16OceanDark => "base16-ocean.dark",
        }
    }
}

/// Syntax highlighting configuration.
#[derive(Debug, Clone)]
pub struct SyntaxConfig {
    /// Whether to show line numbers
    pub line_numbers: bool,
    /// Whether to wrap in a panel
    pub panel: bool,
    /// Border style for panel
    pub border_style: BorderStyle,
    /// Theme to use
    pub theme: Theme,
    /// Starting line number
    pub start_line: usize,
    /// Highlight specific lines
    pub highlight_lines: Vec<usize>,
}

impl Default for SyntaxConfig {
    fn default() -> Self {
        SyntaxConfig {
            line_numbers: true,
            panel: true,
            border_style: BorderStyle::Rounded,
            theme: Theme::Monokai,
            start_line: 1,
            highlight_lines: Vec::new(),
        }
    }
}

/// Syntax-highlighted source code.
#[derive(Debug)]
pub struct Syntax {
    /// Source code
    code: String,
    /// Programming language
    language: String,
    /// Configuration
    config: SyntaxConfig,
}

impl Syntax {
    /// Create a new Syntax instance for highlighting code.
    pub fn new(code: &str, language: &str) -> Self {
        Syntax {
            code: code.to_string(),
            language: language.to_string(),
            config: SyntaxConfig::default(),
        }
    }

    /// Set the configuration.
    pub fn config(mut self, config: SyntaxConfig) -> Self {
        self.config = config;
        self
    }

    /// Set whether to show line numbers.
    pub fn line_numbers(mut self, show: bool) -> Self {
        self.config.line_numbers = show;
        self
    }

    /// Set whether to wrap in a panel.
    pub fn panel(mut self, panel: bool) -> Self {
        self.config.panel = panel;
        self
    }

    /// Set the theme.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.config.theme = theme;
        self
    }

    /// Set lines to highlight.
    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.config.highlight_lines = lines;
        self
    }

    /// Convert syntect style to our Style.
    fn convert_style(syntect_style: SyntectStyle) -> Style {
        Style::new().foreground(Color::rgb(
            syntect_style.foreground.r,
            syntect_style.foreground.g,
            syntect_style.foreground.b,
        ))
    }

    /// Highlight the code and return styled lines.
    fn highlight(&self) -> Vec<Vec<Span>> {
        let syntax_set = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
        let theme_set = THEME_SET.get_or_init(ThemeSet::load_defaults);

        let syntax = syntax_set
            .find_syntax_by_extension(&self.language)
            .or_else(|| syntax_set.find_syntax_by_name(&self.language))
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let theme = theme_set
            .themes
            .get(self.config.theme.name())
            .unwrap_or_else(|| theme_set.themes.values().next().unwrap());

        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut lines = Vec::new();
        let line_number_width = (self.code.lines().count() + self.config.start_line)
            .to_string()
            .len();

        for (i, line) in LinesWithEndings::from(&self.code).enumerate() {
            let line_num = i + self.config.start_line;
            let mut spans = Vec::new();

            // Add line number if enabled
            if self.config.line_numbers {
                let is_highlighted = self.config.highlight_lines.contains(&line_num);
                let line_style = if is_highlighted {
                    Style::new().foreground(Color::Yellow).bold()
                } else {
                    Style::new().foreground(Color::BrightBlack)
                };

                let marker = if is_highlighted { "→ " } else { "  " };
                spans.push(Span::styled(marker.to_string(), line_style));
                spans.push(Span::styled(
                    format!("{:>width$} │ ", line_num, width = line_number_width),
                    line_style,
                ));
            }

            // Highlight the line content
            let highlighted = highlighter
                .highlight_line(line, syntax_set)
                .unwrap_or_default();

            for (style, text) in highlighted {
                let text = text.trim_end_matches('\n').to_string();
                if !text.is_empty() {
                    spans.push(Span::styled(text, Self::convert_style(style)));
                }
            }

            lines.push(spans);
        }

        lines
    }
}

impl Renderable for Syntax {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let highlighted_lines = self.highlight();

        if self.config.panel {
            // Build text content
            let mut text = Text::new();
            for (i, spans) in highlighted_lines.iter().enumerate() {
                for span in spans {
                    text.push_span(span.clone());
                }
                if i < highlighted_lines.len() - 1 {
                    text.push("\n");
                }
            }

            let panel = Panel::new(text)
                .title(&self.language)
                .border_style(self.config.border_style)
                .style(Style::new().foreground(Color::BrightBlack));

            panel.render(context)
        } else {
            highlighted_lines.into_iter().map(Segment::line).collect()
        }
    }
}

/// Convenience function to highlight code inline.
pub fn highlight(code: &str, language: &str) -> Syntax {
    Syntax::new(code, language)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_basic() {
        let syntax = Syntax::new("let x = 42;", "rust");
        let context = RenderContext { width: 60 };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_without_panel() {
        let syntax = Syntax::new("print('hello')", "python").panel(false);
        let context = RenderContext { width: 60 };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_without_line_numbers() {
        let syntax = Syntax::new("x = 1", "python").line_numbers(false);
        let context = RenderContext { width: 60 };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_themes() {
        let syntax = Syntax::new("let x = 42;", "rust").theme(Theme::SolarizedDark);
        let context = RenderContext { width: 60 };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }
}
