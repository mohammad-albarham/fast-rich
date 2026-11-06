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
use syntect::highlighting::{Style as SyntectStyle, Theme as SyntectTheme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
static MONOKAI: OnceLock<SyntectTheme> = OnceLock::new();

/// Monokai, with the colours of Pygments' `monokai` style (Rich's default).
/// syntect's bundled theme set has no Monokai, so it is built here.
fn monokai_theme() -> SyntectTheme {
    use std::str::FromStr;
    use syntect::highlighting::{
        Color as SColor, FontStyle, ScopeSelectors, StyleModifier, ThemeItem, ThemeSettings,
    };

    let rgb = |hex: u32| SColor {
        r: (hex >> 16) as u8,
        g: (hex >> 8) as u8,
        b: hex as u8,
        a: 0xff,
    };
    let (white, pink, cyan, green, yellow, purple, grey) = (
        rgb(0xf8f8f2),
        rgb(0xff4689),
        rgb(0x66d9ef),
        rgb(0xa6e22e),
        rgb(0xe6db74),
        rgb(0xae81ff),
        rgb(0x959077),
    );
    // (TextMate scope selector, colour, italic). Later, more specific
    // selectors win, as in TextMate themes.
    let rules = [
        ("comment", grey, false),
        ("string", yellow, false),
        ("comment.block.documentation, string.quoted.docstring", yellow, false),
        ("storage.type.string", yellow, false),
        ("constant.other.placeholder, punctuation.section.interpolation", yellow, false),
        ("constant.numeric, constant.character, constant.other", purple, false),
        ("constant.language", cyan, false),
        ("keyword", cyan, false),
        ("keyword.operator, keyword.operator.word, punctuation.accessor", pink, false),
        ("keyword.control.import, keyword.other.import, keyword.control.from", pink, false),
        ("storage.type, storage.modifier", cyan, false),
        ("storage.type.function.arrow", white, false),
        ("entity.name.function, support.function.magic", green, false),
        ("entity.name.class, entity.name.type", green, false),
        ("meta.annotation, variable.annotation, entity.name.function.decorator", green, false),
        ("support.macro, entity.name.macro, support.function.macro", green, false),
        ("entity.name.tag", pink, false),
        (
            "meta.mapping.key string, support.type.property-name, meta.structure.dictionary.key string",
            pink,
            false,
        ),
        ("entity.other.attribute-name", green, false),
        ("support.type.exception", green, false),
        ("variable.language, variable.parameter.function.language", green, false),
        // Pygments' JavaScript lexer marks plain identifiers as Name.Other.
        (
            "source.js variable.other, source.js variable.parameter, source.ts variable.other",
            green,
            false,
        ),
        ("variable.parameter, support.function, support.type", white, false),
        ("markup.deleted", pink, false),
        ("markup.inserted", green, false),
        ("markup.italic", white, true),
        ("invalid", pink, false),
    ];
    let scopes = rules
        .iter()
        .filter_map(|(selector, color, italic)| {
            Some(ThemeItem {
                scope: ScopeSelectors::from_str(selector).ok()?,
                style: StyleModifier {
                    foreground: Some(*color),
                    background: None,
                    font_style: italic.then_some(FontStyle::ITALIC),
                },
            })
        })
        .collect();
    SyntectTheme {
        name: Some("monokai".into()),
        settings: ThemeSettings {
            foreground: Some(white),
            background: Some(rgb(0x272822)),
            ..ThemeSettings::default()
        },
        scopes,
        ..SyntectTheme::default()
    }
}

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
            Theme::Monokai => "monokai",
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
///
/// The expensive syntect highlighting pass runs once and is cached; later
/// renders (e.g. inside a `Live` display) reuse it until the language or
/// theme changes.
#[derive(Debug)]
pub struct Syntax {
    /// Source code
    code: String,
    /// Programming language
    language: String,
    /// Configuration
    config: SyntaxConfig,
    /// Cached raw highlighting: (language, theme, per-line styled segments).
    highlight_cache: std::sync::Mutex<Option<HighlightCacheKey>>,
}

/// Cached raw syntect output: (language, theme, per-line styled segments).
type HighlightCacheKey = (String, Theme, Vec<Vec<(SyntectStyle, String)>>);

impl Syntax {
    /// Create a new Syntax instance for highlighting code.
    pub fn new(code: &str, language: &str) -> Self {
        Syntax {
            code: code.to_string(),
            language: language.to_string(),
            config: SyntaxConfig::default(),
            highlight_cache: std::sync::Mutex::new(None),
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
        let mut style = Style::new().foreground(Color::rgb(
            syntect_style.foreground.r,
            syntect_style.foreground.g,
            syntect_style.foreground.b,
        ));

        // Only apply background if it's not transparent/default
        // Syntect ensures colors are RGBA, so we can check alpha or defaults.
        // Usually, code spans don't have distinct backgrounds unless specified by the theme for that token.
        // For now, we'll map it if it differs from the theme default, but syntect converts generic theme styles
        // to specific style objects. Let's apply it if the alpha is sufficient.
        if syntect_style.background.a > 0 {
            style = style.background(Color::rgb(
                syntect_style.background.r,
                syntect_style.background.g,
                syntect_style.background.b,
            ));
        }

        // Font style modifiers
        if syntect_style
            .font_style
            .contains(syntect::highlighting::FontStyle::BOLD)
        {
            style = style.bold();
        }
        if syntect_style
            .font_style
            .contains(syntect::highlighting::FontStyle::ITALIC)
        {
            style = style.italic();
        }
        if syntect_style
            .font_style
            .contains(syntect::highlighting::FontStyle::UNDERLINE)
        {
            style = style.underline();
        }

        style
    }

    /// Highlight the code and return styled lines.
    ///
    /// The raw syntect highlighting is cached; per-render work is limited to
    /// style conversion and gutter construction.
    fn highlight(&self) -> (Vec<Vec<Span>>, Option<Color>, Option<Color>) {
        let syntax_set = SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines);
        let theme_set = THEME_SET.get_or_init(ThemeSet::load_defaults);

        // `find_syntax_by_token` matches extensions and (case-insensitive)
        // names, so both "py" and "python"/"Python" work, like Pygments
        // lexer names in Rich.
        let syntax = syntax_set
            .find_syntax_by_extension(&self.language)
            .or_else(|| syntax_set.find_syntax_by_token(&self.language))
            .or_else(|| syntax_set.find_syntax_by_name(&self.language))
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let theme: &SyntectTheme = match self.config.theme {
            Theme::Monokai => MONOKAI.get_or_init(monokai_theme),
            other => theme_set
                .themes
                .get(other.name())
                .unwrap_or_else(|| MONOKAI.get_or_init(monokai_theme)),
        };

        // Extract global theme background
        let theme_bg = theme.settings.background.map(|c| Color::rgb(c.r, c.g, c.b));

        // Get (cached) raw highlighting for the current language + theme.
        let raw: Vec<Vec<(SyntectStyle, String)>> = {
            let mut cache = self
                .highlight_cache
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if let Some((lang, theme_name, lines)) = cache.as_ref() {
                if lang == &self.language && *theme_name == self.config.theme {
                    lines.clone()
                } else {
                    let computed = self.highlight_raw(syntax, theme, syntax_set);
                    *cache = Some((self.language.clone(), self.config.theme, computed.clone()));
                    computed
                }
            } else {
                let computed = self.highlight_raw(syntax, theme, syntax_set);
                *cache = Some((self.language.clone(), self.config.theme, computed.clone()));
                computed
            }
        };

        let mut lines: Vec<Vec<Span>> = raw
            .iter()
            .map(|segments| {
                segments
                    .iter()
                    .filter(|(_, text)| !text.is_empty())
                    .map(|(style, text)| Span::styled(text.clone(), Self::convert_style(*style)))
                    .collect()
            })
            .collect();
        // Like Rich, code ending in a newline shows a final empty line.
        if self.code.ends_with('\n') {
            lines.push(Vec::new());
        }
        let theme_fg = theme.settings.foreground.map(|c| Color::rgb(c.r, c.g, c.b));
        (lines, theme_bg, theme_fg)
    }

    /// Width of the line-number column, as in Rich's
    /// `Syntax._numbers_column_width` (0 without line numbers).
    fn numbers_column_width(&self) -> usize {
        if !self.config.line_numbers {
            return 0;
        }
        (self.config.start_line + self.code.lines().count())
            .to_string()
            .len()
            + 2
    }

    /// Widest source line in cells (tabs count as four spaces).
    fn longest_line(&self) -> usize {
        self.code
            .lines()
            .map(|l| unicode_width::UnicodeWidthStr::width(l.replace('\t', "    ").as_str()))
            .max()
            .unwrap_or(0)
    }

    /// Rich's `Syntax` layout: a gutter (`❱` marker, right-aligned number,
    /// space) and the code padded with the theme background to the width.
    fn render_lines_rich(&self, width: usize) -> Vec<Vec<Span>> {
        let (lines, theme_bg, theme_fg) = self.highlight();
        let background = theme_bg.map_or(Style::new(), |bg| Style::new().background(bg));
        let numbers = self.numbers_column_width();
        let code_width = if self.config.line_numbers {
            width.saturating_sub(numbers + 1)
        } else {
            width
        };
        // Line numbers: Rich blends the text colour into the background
        // (30%, or 90% and bold for the highlighted line).
        let blend = |amount: f64| match (theme_bg, theme_fg) {
            (
                Some(Color::Rgb { r, g, b }),
                Some(Color::Rgb {
                    r: fr,
                    g: fg,
                    b: fb,
                }),
            ) => {
                let mix =
                    |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * amount) as u8;
                Some(Color::rgb(mix(r, fr), mix(g, fg), mix(b, fb)))
            }
            _ => theme_fg,
        };
        let number_style = background.combine(&Style {
            foreground: blend(0.3),
            dim: theme_bg.is_none(),
            ..Style::new()
        });
        let highlight_number_style = background.combine(&Style {
            foreground: blend(0.9),
            bold: true,
            ..Style::new()
        });
        lines
            .into_iter()
            .enumerate()
            .map(|(i, spans)| {
                let line_no = i + self.config.start_line;
                let mut out = Vec::new();
                if self.config.line_numbers {
                    let column = format!("{:>w$} ", line_no, w = numbers.saturating_sub(2));
                    if self.config.highlight_lines.contains(&line_no) {
                        out.push(Span::styled("❱ ", Style::new().foreground(Color::Red)));
                        out.push(Span::styled(column, highlight_number_style));
                    } else {
                        out.push(Span::styled("  ", highlight_number_style));
                        out.push(Span::styled(column, number_style));
                    }
                }
                let code: Vec<Span> = spans
                    .into_iter()
                    .map(|mut span| {
                        span.style = background.combine(&span.style);
                        span
                    })
                    .collect();
                out.extend(crate::renderable::fit_line(code, code_width, background));
                out
            })
            .collect()
    }

    /// Run syntect over the source once and collect raw styled segments.
    fn highlight_raw(
        &self,
        syntax: &syntect::parsing::SyntaxReference,
        theme: &SyntectTheme,
        syntax_set: &SyntaxSet,
    ) -> Vec<Vec<(SyntectStyle, String)>> {
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut lines = Vec::new();

        for line in LinesWithEndings::from(&self.code) {
            // Syntect handles syntax by line, but doesn't expand tabs.
            // We expand tabs to spaces to keep width calculations consistent.
            let line = line.replace('\t', "    ");
            let highlighted = highlighter
                .highlight_line(&line, syntax_set)
                .unwrap_or_default();

            lines.push(
                highlighted
                    .into_iter()
                    .map(|(style, text)| (style, text.trim_end_matches('\n').to_string()))
                    .collect(),
            );
        }

        lines
    }
}

impl Renderable for Syntax {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        if self.config.panel {
            // Panel around the code: lay the code out at the inner width.
            let inner = context.width.saturating_sub(4);
            let mut text = Text::new();
            let lines = self.render_lines_rich(inner.min(self.max_width()));
            let count = lines.len();
            for (i, spans) in lines.into_iter().enumerate() {
                for span in spans {
                    text.push_span(span);
                }
                if i + 1 < count {
                    text.push("\n");
                }
            }
            text.overflow = crate::text::Overflow::Truncate;
            Panel::new(text)
                .title(&self.language)
                .border_style(self.config.border_style)
                .style(Style::new().foreground(Color::BrightBlack))
                .render(context)
        } else {
            self.render_lines_rich(context.width)
                .into_iter()
                .map(Segment::line)
                .collect()
        }
    }

    fn min_width(&self) -> usize {
        self.numbers_column_width()
    }

    /// Rich's measurement: gutter + longest line (+1 with line numbers).
    fn max_width(&self) -> usize {
        let gutter = self.numbers_column_width() + usize::from(self.config.line_numbers);
        let inner = gutter + self.longest_line();
        if self.config.panel {
            inner + 4
        } else {
            inner
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
        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_without_panel() {
        let syntax = Syntax::new("print('hello')", "python").panel(false);
        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_without_line_numbers() {
        let syntax = Syntax::new("x = 1", "python").line_numbers(false);
        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_syntax_themes() {
        let syntax = Syntax::new("let x = 42;", "rust").theme(Theme::SolarizedDark);
        let context = RenderContext {
            width: 60,
            height: None,
        };
        let segments = syntax.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_language_name_lookup_is_case_insensitive() {
        let context = RenderContext {
            width: 40,
            height: None,
        };
        let colored = |lang: &str| {
            Syntax::new("def f():\n    return 1\n", lang)
                .line_numbers(false)
                .panel(false)
                .render(&context)
                .iter()
                .flat_map(|s| s.spans.iter())
                .any(|s| s.style.foreground.is_some())
        };
        assert!(colored("python"), "lowercase language name must highlight");
        assert!(colored("Python"));
        assert!(colored("py"));
    }

    fn plain(syntax: &Syntax, width: usize) -> Vec<String> {
        syntax
            .render(&RenderContext {
                width,
                height: None,
            })
            .iter()
            .map(|s| s.plain_text().trim_end().to_string())
            .collect()
    }

    #[test]
    fn test_every_theme_resolves_to_its_own_colours() {
        // Monokai used to name a theme syntect doesn't ship and fell back
        // to a light theme (white background).
        let bg = |theme: Theme| {
            let syntax = Syntax::new("x = 1\n", "python").panel(false).theme(theme);
            syntax.highlight().1
        };
        assert_eq!(bg(Theme::Monokai), Some(Color::rgb(0x27, 0x28, 0x22)));
        for theme in [
            Theme::InspiredGitHub,
            Theme::SolarizedDark,
            Theme::SolarizedLight,
            Theme::Base16OceanDark,
        ] {
            assert_ne!(
                bg(theme),
                bg(Theme::Monokai),
                "{theme:?} fell back to Monokai"
            );
        }
    }

    #[test]
    fn test_layout_matches_rich() {
        let code = "def fib(n):\n    return n\n";
        let syntax = Syntax::new(code, "python")
            .panel(false)
            .highlight_lines(vec![2]);
        // Rich: `  1 def…`, `❱ 2     return n`, and a final empty line 3.
        assert_eq!(
            plain(&syntax, 30),
            ["  1 def fib(n):", "❱ 2     return n", "  3"]
        );
        // Rich's measurement: numbers column (1 digit + 2) + 1 + longest
        // line ("    return n", 12 cells).
        assert_eq!(syntax.max_width(), 3 + 1 + 12);
        let lines = syntax.render(&RenderContext {
            width: 30,
            height: None,
        });
        assert!(
            lines.iter().all(|l| l.width() == 30),
            "code area is padded to the width"
        );
    }
}
