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
static ANSI_DARK: OnceLock<SyntectTheme> = OnceLock::new();
static ANSI_LIGHT: OnceLock<SyntectTheme> = OnceLock::new();

/// syntect colours can only be RGB. ANSI themes store a terminal colour
/// index in an otherwise impossible fully transparent colour: `r` is the
/// index (0-15, or 255 for "no colour"), `g` bit 0 means dim.
const ANSI_NONE: u8 = 255;

fn ansi_marker(index: u8, dim: bool) -> syntect::highlighting::Color {
    syntect::highlighting::Color {
        r: index,
        g: u8::from(dim),
        b: 0,
        a: 0,
    }
}

/// Decode an ANSI marker colour (see [`ansi_marker`]).
fn decode_ansi(c: syntect::highlighting::Color) -> Option<(Option<Color>, bool)> {
    if c.a != 0 {
        return None;
    }
    const NAMES: [Color; 16] = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightBlack,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];
    Some((NAMES.get(usize::from(c.r)).copied(), c.g & 1 == 1))
}

/// Rich's `ANSI_DARK` / `ANSI_LIGHT` token styles mapped to TextMate scopes.
fn ansi_theme(dark: bool) -> SyntectTheme {
    use std::str::FromStr;
    use syntect::highlighting::{
        FontStyle, ScopeSelectors, StyleModifier, ThemeItem, ThemeSettings,
    };

    // ANSI indexes: normal 0-7, bright = +8.
    let shade = |normal: u8| if dark { normal + 8 } else { normal };
    let (red, green, yellow, blue, magenta, cyan) = (1, 2, 3, 4, 5, 6);
    let none = FontStyle::empty();
    let rules: [(&str, u8, bool, FontStyle); 18] = [
        ("comment", ANSI_NONE, true, none),
        (
            "meta.preprocessor, keyword.control.directive",
            shade(cyan),
            false,
            none,
        ),
        (
            "keyword, storage.type, storage.modifier, constant.language",
            shade(blue),
            false,
            none,
        ),
        (
            "keyword.operator.word, keyword.operator.logical, keyword.control.flow.for.in",
            shade(magenta),
            false,
            none,
        ),
        ("keyword.operator", ANSI_NONE, false, none),
        (
            "support.function.builtin, support.type, variable.language",
            shade(cyan),
            false,
            none,
        ),
        (
            "entity.name.function, support.function.magic",
            shade(green),
            false,
            none,
        ),
        (
            "entity.name.class, entity.name.type.class",
            shade(green),
            false,
            FontStyle::UNDERLINE,
        ),
        ("support.type.exception", shade(cyan), false, none),
        (
            "meta.annotation, variable.annotation, entity.name.function.decorator",
            shade(magenta),
            false,
            FontStyle::BOLD,
        ),
        ("entity.other.attribute-name", shade(cyan), false, none),
        ("entity.name.tag", shade(blue), false, none),
        (
            "string, storage.type.string, constant.other.placeholder",
            yellow,
            false,
            none,
        ),
        ("constant.numeric", shade(blue), false, none),
        ("markup.deleted", shade(red), false, none),
        ("markup.inserted", shade(green), false, none),
        ("markup.heading", ANSI_NONE, false, FontStyle::BOLD),
        ("invalid", red, false, FontStyle::UNDERLINE),
    ];
    let scopes = rules
        .iter()
        .filter_map(|(selector, index, dim, font)| {
            Some(ThemeItem {
                scope: ScopeSelectors::from_str(selector).ok()?,
                style: StyleModifier {
                    foreground: Some(ansi_marker(*index, *dim)),
                    background: None,
                    font_style: Some(*font),
                },
            })
        })
        .collect();
    SyntectTheme {
        name: Some(if dark { "ansi_dark" } else { "ansi_light" }.into()),
        settings: ThemeSettings {
            foreground: Some(ansi_marker(ANSI_NONE, false)),
            background: Some(ansi_marker(ANSI_NONE, false)),
            ..ThemeSettings::default()
        },
        scopes,
        ..SyntectTheme::default()
    }
}

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
        (
            "keyword.operator, keyword.operator.word, punctuation.accessor, keyword.control.flow.for.in",
            pink,
            false,
        ),
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
    /// The terminal's own 16 colours on its background, bright variants
    /// (Rich's `ansi_dark`, the default for tracebacks)
    AnsiDark,
    /// The terminal's own 16 colours on its background, normal variants
    /// (Rich's `ansi_light`)
    AnsiLight,
}

impl Theme {
    fn syntect(&self) -> &'static SyntectTheme {
        let theme_set = THEME_SET.get_or_init(ThemeSet::load_defaults);
        match self {
            Theme::Monokai => MONOKAI.get_or_init(monokai_theme),
            Theme::AnsiDark => ANSI_DARK.get_or_init(|| ansi_theme(true)),
            Theme::AnsiLight => ANSI_LIGHT.get_or_init(|| ansi_theme(false)),
            other => theme_set
                .themes
                .get(other.name())
                .unwrap_or_else(|| MONOKAI.get_or_init(monokai_theme)),
        }
    }

    /// The theme's background colour.
    pub fn background(&self) -> Option<Color> {
        self.syntect()
            .settings
            .background
            .filter(|c| c.a > 0)
            .map(|c| Color::rgb(c.r, c.g, c.b))
    }

    /// The style the theme gives a TextMate scope such as `"string"`,
    /// `"constant.numeric"` or `"entity.name.function"`.
    ///
    /// ```
    /// use fast_rich::syntax::Theme;
    /// use fast_rich::style::Color;
    /// assert_eq!(
    ///     Theme::Monokai.style_for_scope("string").foreground,
    ///     Some(Color::rgb(0xe6, 0xdb, 0x74))
    /// );
    /// ```
    pub fn style_for_scope(&self, scope: &str) -> Style {
        use std::str::FromStr;
        use syntect::highlighting::Highlighter;
        use syntect::parsing::ScopeStack;
        let highlighter = Highlighter::new(self.syntect());
        match ScopeStack::from_str(scope) {
            Ok(stack) => {
                let mut style =
                    Syntax::convert_style(highlighter.style_for_stack(stack.as_slice()));
                // Token styles only; the background is `background()`.
                style.background = None;
                style
            }
            Err(_) => Style::new(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Theme::Monokai => "monokai",
            Theme::InspiredGitHub => "InspiredGitHub",
            Theme::SolarizedDark => "Solarized (dark)",
            Theme::SolarizedLight => "Solarized (light)",
            Theme::Base16OceanDark => "base16-ocean.dark",
            Theme::AnsiDark => "ansi_dark",
            Theme::AnsiLight => "ansi_light",
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
    /// Draw `│` guides in the indentation (Rich's `indent_guides`)
    pub indent_guides: bool,
    /// Spaces per indentation level (and per tab)
    pub tab_size: usize,
    /// Only show lines `start..=end` (1-based, like Rich's `line_range`);
    /// the whole file is still highlighted, so context is right
    pub line_range: Option<(usize, usize)>,
    /// Fixed width of the code area (Rich's `code_width`)
    pub code_width: Option<usize>,
    /// Extra styles over character ranges: (line, start col, end col, style),
    /// lines numbered like the gutter, columns as character offsets
    pub ranges: Vec<(usize, usize, usize, Style)>,
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
            indent_guides: false,
            tab_size: 4,
            line_range: None,
            code_width: None,
            ranges: Vec::new(),
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

    /// Show only lines `start..=end` (1-based), like Rich's `line_range`.
    pub fn line_range(mut self, start: usize, end: usize) -> Self {
        self.config.line_range = Some((start, end));
        self
    }

    /// Fix the width of the code area, like Rich's `code_width`.
    pub fn code_width(mut self, width: usize) -> Self {
        self.config.code_width = Some(width);
        self
    }

    /// Draw `│` indentation guides (Rich's `indent_guides`).
    pub fn indent_guides(mut self, guides: bool) -> Self {
        self.config.indent_guides = guides;
        self
    }

    /// Apply `style` from `start` to `end` (both `(line, column)`, line
    /// numbers as in the gutter, columns as character offsets), like
    /// Rich's `Syntax.stylize_range`.
    pub fn stylize_range(
        mut self,
        style: Style,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Self {
        for line in start.0..=end.0 {
            let from = if line == start.0 { start.1 } else { 0 };
            let to = if line == end.0 { end.1 } else { usize::MAX };
            if from < to {
                self.config.ranges.push((line, from, to, style));
            }
        }
        self
    }

    /// Set lines to highlight.
    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.config.highlight_lines = lines;
        self
    }

    /// Convert syntect style to our Style.
    fn convert_style(syntect_style: SyntectStyle) -> Style {
        let mut style = match decode_ansi(syntect_style.foreground) {
            // ANSI theme: terminal colour (or none), maybe dim.
            Some((color, dim)) => Style {
                foreground: color,
                dim,
                ..Style::new()
            },
            None => Style::new().foreground(Color::rgb(
                syntect_style.foreground.r,
                syntect_style.foreground.g,
                syntect_style.foreground.b,
            )),
        };

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

        // `find_syntax_by_token` matches extensions and (case-insensitive)
        // names, so both "py" and "python"/"Python" work, like Pygments
        // lexer names in Rich.
        let syntax = syntax_set
            .find_syntax_by_extension(&self.language)
            .or_else(|| syntax_set.find_syntax_by_token(&self.language))
            .or_else(|| syntax_set.find_syntax_by_name(&self.language))
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        let theme: &SyntectTheme = self.config.theme.syntect();

        // Extract global theme background
        let theme_bg = theme
            .settings
            .background
            .filter(|c| c.a > 0)
            .map(|c| Color::rgb(c.r, c.g, c.b));

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
        let theme_fg = theme
            .settings
            .foreground
            .and_then(|c| match decode_ansi(c) {
                Some((color, _)) => color,
                None => Some(Color::rgb(c.r, c.g, c.b)),
            });
        (lines, theme_bg, theme_fg)
    }

    /// Highlighted code lines (no gutter) and the theme background.
    pub(crate) fn highlighted_lines(&self) -> (Vec<Vec<Span>>, Option<Color>) {
        let (lines, bg, _) = self.highlight();
        (lines, bg)
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
        let code_width = self
            .config
            .code_width
            .unwrap_or(if self.config.line_numbers {
                width.saturating_sub(numbers + 1)
            } else {
                width
            });
        // Rich: lines[line_offset:end_line], numbered from start_line + offset.
        let line_offset = self
            .config
            .line_range
            .map_or(0, |(start, _)| start.saturating_sub(1));
        let line_end = self
            .config
            .line_range
            .map_or(lines.len(), |(_, end)| end.min(lines.len()));
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
        // Transparent (ANSI) themes: Rich uses dim numbers and a plain
        // highlighted number.
        let (number_style, highlight_number_style) = if theme_bg.is_none() {
            (Style::new().dim(), Style::new())
        } else {
            (
                background.combine(&Style {
                    foreground: blend(0.3),
                    ..Style::new()
                }),
                background.combine(&Style {
                    foreground: blend(0.9),
                    bold: true,
                    ..Style::new()
                }),
            )
        };
        // Rich's Text.with_indent_guides: "│" + spaces per indentation
        // level; blank lines take the indentation of the next line.
        let guide_style = background.combine(
            &self
                .config
                .theme
                .style_for_scope("comment")
                .combine(&Style::new().dim()),
        );
        let guides: Vec<String> = if self.config.indent_guides {
            indent_guides(&lines, self.config.tab_size.max(1))
        } else {
            Vec::new()
        };
        lines
            .into_iter()
            .enumerate()
            .skip(line_offset)
            .take(line_end.saturating_sub(line_offset))
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
                let mut spans = spans;
                for (line, from, to, style) in &self.config.ranges {
                    if *line == line_no {
                        spans = restyle(spans, *from, *to, *style);
                    }
                }
                if let Some(indent) = guides.get(i) {
                    spans = replace_indent(spans, indent, guide_style);
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

/// Indentation guide strings per line (Rich's `Text.with_indent_guides`).
fn indent_guides(lines: &[Vec<Span>], indent_size: usize) -> Vec<String> {
    let mut out = vec![String::new(); lines.len()];
    let mut pending_blank: Vec<usize> = Vec::new();
    for (i, spans) in lines.iter().enumerate() {
        let plain: String = spans.iter().map(|s| s.text.as_ref()).collect();
        let indent = plain.len() - plain.trim_start_matches(' ').len();
        if plain.trim().is_empty() {
            pending_blank.push(i);
            continue;
        }
        let guide = format!(
            "{}{}",
            format!("│{}", " ".repeat(indent_size - 1)).repeat(indent / indent_size),
            " ".repeat(indent % indent_size)
        );
        for b in pending_blank.drain(..) {
            out[b] = guide.clone();
        }
        out[i] = guide;
    }
    out
}

/// Replace the first `guide.chars().count()` characters of a line with the
/// guide (in `style`), keeping the rest of the spans.
fn replace_indent(spans: Vec<Span>, guide: &str, style: Style) -> Vec<Span> {
    let n = guide.chars().count();
    if n == 0 {
        return spans;
    }
    let plain_len: usize = spans.iter().map(|s| s.text.chars().count()).sum();
    let mut out = vec![Span::styled(guide.to_string(), style)];
    if plain_len > n {
        out.extend(restyle_split(spans, n));
    }
    out
}

/// Spans after the first `skip` characters.
fn restyle_split(spans: Vec<Span>, skip: usize) -> Vec<Span> {
    let mut seen = 0;
    let mut out = Vec::new();
    for span in spans {
        let len = span.text.chars().count();
        if seen + len <= skip {
            seen += len;
            continue;
        }
        let cut = skip.saturating_sub(seen);
        let text: String = span.text.chars().skip(cut).collect();
        out.push(Span {
            text: text.into(),
            style: span.style,
            link: span.link,
        });
        seen += len;
    }
    out
}

/// Combine `style` over characters `from..to` of a line.
fn restyle(spans: Vec<Span>, from: usize, to: usize, style: Style) -> Vec<Span> {
    let mut out = Vec::new();
    let mut pos = 0;
    for span in spans {
        let chars: Vec<char> = span.text.chars().collect();
        let (start, end) = (pos, pos + chars.len());
        pos = end;
        let (a, b) = (from.clamp(start, end), to.clamp(start, end));
        if a >= b {
            out.push(span);
            continue;
        }
        let piece = |x: usize, y: usize, st: Style| Span {
            text: chars[x - start..y - start]
                .iter()
                .collect::<String>()
                .into(),
            style: st,
            link: span.link.clone(),
        };
        if a > start {
            out.push(piece(start, a, span.style));
        }
        out.push(piece(a, b, span.style.combine(&style)));
        if b < end {
            out.push(piece(b, end, span.style));
        }
    }
    out
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
        let inner = match self.config.code_width {
            // Rich: code_width + numbers column + 1
            Some(w) => w + self.numbers_column_width() + 1,
            None => {
                self.numbers_column_width()
                    + usize::from(self.config.line_numbers)
                    + self.longest_line()
            }
        };
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
