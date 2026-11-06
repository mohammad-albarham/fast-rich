//! Horizontal rules for visual separation.
//!
//! A `Rule` draws a horizontal line across the terminal, optionally with
//! a centered title.

use crate::console::RenderContext;
use crate::renderable::{line_width, Renderable, Segment};
use crate::style::Style;
use crate::text::{Alignment, Overflow, Span, Text};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A horizontal rule/line.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Optional title (markup parsed)
    title: Option<Text>,
    /// Characters repeated to draw the line
    characters: String,
    /// Style for the rule
    style: Style,
    /// Style for the title
    title_style: Style,
    /// Alignment of the title
    align: crate::text::Alignment,
}

/// Alignment for the rule title.
///
/// Deprecated alias of [`Alignment`] — use that
/// instead; the four alignment enums are being consolidated.
#[deprecated(since = "0.3.3", note = "use `fast_rich::text::Alignment` instead")]
pub type RuleAlign = crate::text::Alignment;

impl Rule {
    /// Create a new rule with optional title.
    pub fn new(title: &str) -> Self {
        Rule {
            title: if title.is_empty() {
                None
            } else {
                Some(crate::markup::parse(title))
            },
            characters: "─".to_string(),
            style: Style::new(),
            title_style: Style::new(),
            align: crate::text::Alignment::Center,
        }
    }

    /// Create a rule without a title.
    pub fn line() -> Self {
        Rule {
            title: None,
            characters: "─".to_string(),
            style: Style::new(),
            title_style: Style::new(),
            align: crate::text::Alignment::Center,
        }
    }

    /// Set the character used for the line.
    pub fn character(mut self, c: char) -> Self {
        self.characters = c.to_string();
        self
    }

    /// Use a string of one or more characters for the line (Rich's
    /// `characters`), repeated and cut to the width.
    pub fn characters(mut self, characters: &str) -> Self {
        if !characters.is_empty() {
            self.characters = characters.to_string();
        }
        self
    }

    /// Use styled text as the title (markup already parsed).
    pub fn title_text(mut self, title: Text) -> Self {
        self.title = if title.plain_text().is_empty() {
            None
        } else {
            Some(title)
        };
        self
    }

    /// Set the style for the rule line.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the style for the title.
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set the title alignment.
    pub fn align(mut self, align: crate::text::Alignment) -> Self {
        self.align = align;
        self
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule::line()
    }
}

/// `characters` repeated to cover exactly `width` cells.
fn repeat_to_width(characters: &str, width: usize) -> String {
    let chars_len = UnicodeWidthStr::width(characters).max(1);
    let repeated = characters.repeat(width / chars_len + 1);
    set_cell_size(&repeated, width)
}

/// Crop or pad `s` to exactly `width` cells (Rich's `set_cell_size`).
fn set_cell_size(s: &str, width: usize) -> String {
    let mut out = String::new();
    let mut used = 0;
    for g in s.graphemes(true) {
        let w = UnicodeWidthStr::width(g);
        if used + w > width {
            break;
        }
        used += w;
        out.push_str(g);
    }
    out.push_str(&" ".repeat(width - used));
    out
}

impl Renderable for Rule {
    /// Rich's `Rule.__rich_console__`.
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let width = context.width;
        let line = |n: usize| Span::styled(repeat_to_width(&self.characters, n), self.style);
        let title = match &self.title {
            Some(t) => t,
            None => return vec![Segment::line(vec![line(width)])],
        };
        let required = if self.align == Alignment::Center {
            4
        } else {
            2
        };
        let truncate_width = width.saturating_sub(required);
        if truncate_width == 0 {
            return vec![Segment::line(vec![line(width)])];
        }

        // Title on one line, cut with an ellipsis to fit.
        let mut title = title.clone();
        for span in &mut title.spans {
            if span.text.contains('\n') {
                span.text = span.text.replace('\n', " ").into();
            }
            span.style = self.title_style.combine(&span.style);
        }
        title.overflow = Overflow::Ellipsis;
        let title_spans: Vec<Span> = title
            .wrap(truncate_width)
            .into_iter()
            .next()
            .unwrap_or_default();
        let title_width = line_width(&title_spans);

        let mut spans = Vec::new();
        match self.align {
            Alignment::Center => {
                let side = (width - title_width) / 2;
                let left = repeat_to_width(&self.characters, side.saturating_sub(1));
                let right_len =
                    width.saturating_sub(UnicodeWidthStr::width(left.as_str()) + title_width);
                spans.push(Span::styled(format!("{left} "), self.style));
                spans.extend(title_spans);
                let right = repeat_to_width(&self.characters, right_len.saturating_sub(1));
                spans.push(Span::styled(format!(" {right}"), self.style));
            }
            Alignment::Left => {
                spans.extend(title_spans);
                spans.push(Span::raw(" "));
                spans.push(line(width.saturating_sub(title_width + 1)));
            }
            Alignment::Right => {
                spans.push(line(width.saturating_sub(title_width + 1)));
                spans.push(Span::raw(" "));
                spans.extend(title_spans);
            }
        }
        vec![Segment::line(crate::renderable::fit_line(
            spans,
            width,
            Style::new(),
        ))]
    }

    fn min_width(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_simple() {
        let rule = Rule::line();
        let context = RenderContext {
            width: 10,
            height: None,
        };
        let segments = rule.render(&context);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].plain_text(), "──────────");
    }

    #[test]
    fn test_rule_with_title() {
        let rule = Rule::new("Title");
        let context = RenderContext {
            width: 20,
            height: None,
        };
        let segments = rule.render(&context);
        let text = segments[0].plain_text();
        assert!(text.contains("Title"));
        assert!(text.starts_with("─"));
        assert!(text.ends_with("─"));
    }

    #[test]
    fn test_rule_custom_char() {
        let rule = Rule::line().character('=');
        let context = RenderContext {
            width: 5,
            height: None,
        };
        let segments = rule.render(&context);
        assert_eq!(segments[0].plain_text(), "=====");
    }

    #[test]
    fn test_matches_rich() {
        let render = |rule: Rule| {
            let context = RenderContext {
                width: 20,
                height: None,
            };
            let segs = rule.render(&context);
            assert!(
                segs.iter().all(|s| s.newline),
                "rule lines end with a newline"
            );
            segs.iter().map(|s| s.plain_text()).collect::<String>()
        };
        assert_eq!(render(Rule::line()), "────────────────────");
        assert_eq!(render(Rule::new("Title")), "────── Title ───────");
        assert_eq!(
            render(Rule::new("Left").align(Alignment::Left)),
            "Left ───────────────"
        );
        assert_eq!(
            render(Rule::new("R").align(Alignment::Right)),
            "────────────────── R"
        );
        assert_eq!(
            render(Rule::new("T").characters("=-")),
            "=-=-=-=- T =-=-=-=-="
        );
        assert_eq!(
            render(Rule::new("a very long title that will not fit")),
            "─ a very long tit… ─"
        );
    }
}
