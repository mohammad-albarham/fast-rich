//! Markdown rendering for terminal output.
//!
//! This module provides markdown parsing and rendering using pulldown-cmark.
//! It's feature-gated behind the `markdown` feature.

use crate::console::RenderContext;
use crate::panel::{BorderStyle, Panel};
use crate::renderable::{Renderable, Segment};
use crate::rule::Rule;
use crate::style::{Color, Style};
use crate::text::{Span, Text};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

/// Markdown rendering configuration.
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// Style for code blocks
    pub code_style: Style,
    /// Style for inline code
    pub inline_code_style: Style,
    /// Style for headings
    pub heading_styles: [Style; 6],
    /// Style for emphasis (italic)
    pub emphasis_style: Style,
    /// Style for strong (bold)
    pub strong_style: Style,
    /// Style for links
    pub link_style: Style,
    /// Style for blockquotes
    pub quote_style: Style,
    /// Whether to use a panel for code blocks
    pub code_block_panel: bool,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig {
            code_style: Style::new().foreground(Color::BrightBlack),
            inline_code_style: Style::new().foreground(Color::Cyan),
            heading_styles: [
                Style::new().foreground(Color::Magenta).bold(), // H1
                Style::new().foreground(Color::Blue).bold(),    // H2
                Style::new().foreground(Color::Cyan).bold(),    // H3
                Style::new().foreground(Color::Green).bold(),   // H4
                Style::new().foreground(Color::Yellow).bold(),  // H5
                Style::new().bold(),                            // H6
            ],
            emphasis_style: Style::new().italic(),
            strong_style: Style::new().bold(),
            link_style: Style::new().foreground(Color::Blue).underline(),
            quote_style: Style::new().foreground(Color::BrightBlack).italic(),
            code_block_panel: true,
        }
    }
}

/// Rendered markdown content.
#[derive(Debug, Clone)]
pub struct Markdown {
    /// The markdown source
    source: String,
    /// Configuration
    config: MarkdownConfig,
}

impl Markdown {
    /// Create a new Markdown from source text.
    pub fn new(source: &str) -> Self {
        Markdown {
            source: source.to_string(),
            config: MarkdownConfig::default(),
        }
    }

    /// Set the rendering configuration.
    pub fn config(mut self, config: MarkdownConfig) -> Self {
        self.config = config;
        self
    }

    /// Parse the markdown and return rendering events.
    fn parse(&self) -> impl Iterator<Item = MarkdownElement> + '_ {
        let parser = Parser::new(&self.source);
        let mut elements = Vec::new();
        let mut style_stack: Vec<Style> = Vec::new();
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut code_block_lang = String::new();
        let mut list_depth = 0;
        let mut ordered_list_num: Option<u64> = None;

        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Heading { level, .. } => {
                        let level_idx = match level {
                            HeadingLevel::H1 => 0,
                            HeadingLevel::H2 => 1,
                            HeadingLevel::H3 => 2,
                            HeadingLevel::H4 => 3,
                            HeadingLevel::H5 => 4,
                            HeadingLevel::H6 => 5,
                        };
                        style_stack.push(self.config.heading_styles[level_idx]);
                        elements.push(MarkdownElement::StartHeading(level_idx));
                    }
                    Tag::Paragraph => {
                        elements.push(MarkdownElement::StartParagraph);
                    }
                    Tag::Emphasis => {
                        style_stack.push(self.config.emphasis_style);
                    }
                    Tag::Strong => {
                        style_stack.push(self.config.strong_style);
                    }
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_block_content.clear();
                        code_block_lang = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                            pulldown_cmark::CodeBlockKind::Indented => String::new(),
                        };
                    }
                    Tag::Link {
                        dest_url, title: _, ..
                    } => {
                        style_stack.push(self.config.link_style);
                        elements.push(MarkdownElement::StartLink(dest_url.to_string()));
                    }
                    Tag::List(start) => {
                        list_depth += 1;
                        ordered_list_num = start;
                    }
                    Tag::Item => {
                        let prefix = if let Some(num) = ordered_list_num {
                            ordered_list_num = Some(num + 1);
                            format!("{}. ", num)
                        } else {
                            "• ".to_string()
                        };
                        elements.push(MarkdownElement::ListItem {
                            depth: list_depth,
                            prefix,
                        });
                    }
                    Tag::BlockQuote(_) => {
                        style_stack.push(self.config.quote_style);
                        elements.push(MarkdownElement::StartBlockQuote);
                    }
                    _ => {}
                },
                Event::End(tag) => match tag {
                    TagEnd::Heading(_) => {
                        style_stack.pop();
                        elements.push(MarkdownElement::EndHeading);
                    }
                    TagEnd::Paragraph => {
                        elements.push(MarkdownElement::EndParagraph);
                    }
                    TagEnd::Emphasis | TagEnd::Strong => {
                        style_stack.pop();
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        elements.push(MarkdownElement::CodeBlock {
                            content: std::mem::take(&mut code_block_content),
                            language: std::mem::take(&mut code_block_lang),
                        });
                    }
                    TagEnd::Link => {
                        style_stack.pop();
                        elements.push(MarkdownElement::EndLink);
                    }
                    TagEnd::List(_) => {
                        list_depth -= 1;
                        ordered_list_num = None;
                    }
                    TagEnd::Item => {}
                    TagEnd::BlockQuote(_) => {
                        style_stack.pop();
                        elements.push(MarkdownElement::EndBlockQuote);
                    }
                    _ => {}
                },
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        let style = style_stack
                            .iter()
                            .fold(Style::new(), |acc, s| acc.combine(s));
                        elements.push(MarkdownElement::Text(text.to_string(), style));
                    }
                }
                Event::Code(code) => {
                    elements.push(MarkdownElement::InlineCode(code.to_string()));
                }
                Event::SoftBreak => {
                    elements.push(MarkdownElement::SoftBreak);
                }
                Event::HardBreak => {
                    elements.push(MarkdownElement::HardBreak);
                }
                Event::Rule => {
                    elements.push(MarkdownElement::HorizontalRule);
                }
                _ => {}
            }
        }

        elements.into_iter()
    }
}

/// Internal markdown element for rendering.
#[derive(Debug, Clone)]
enum MarkdownElement {
    StartHeading(usize),
    EndHeading,
    StartParagraph,
    EndParagraph,
    Text(String, Style),
    InlineCode(String),
    CodeBlock { content: String, language: String },
    StartLink(String),
    EndLink,
    ListItem { depth: usize, prefix: String },
    StartBlockQuote,
    EndBlockQuote,
    SoftBreak,
    HardBreak,
    HorizontalRule,
}

impl Renderable for Markdown {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut current_line: Vec<Span> = Vec::new();
        let mut _in_heading = false;
        let mut heading_level = 0;

        for element in self.parse() {
            match element {
                MarkdownElement::StartHeading(level) => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                    _in_heading = true;
                    heading_level = level;
                    // Add heading prefix
                    let prefix = "#".repeat(level + 1) + " ";
                    current_line.push(Span::styled(prefix, self.config.heading_styles[level]));
                }
                MarkdownElement::EndHeading => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                    // Add underline for H1/H2
                    if heading_level == 0 {
                        segments.push(Segment::line(vec![Span::styled(
                            "═".repeat(context.width.min(60)),
                            self.config.heading_styles[0],
                        )]));
                    } else if heading_level == 1 {
                        segments.push(Segment::line(vec![Span::styled(
                            "─".repeat(context.width.min(40)),
                            self.config.heading_styles[1],
                        )]));
                    }
                    _in_heading = false;
                    segments.push(Segment::line(vec![])); // Blank line
                }
                MarkdownElement::StartParagraph => {}
                MarkdownElement::EndParagraph => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                    segments.push(Segment::line(vec![])); // Blank line
                }
                MarkdownElement::Text(text, style) => {
                    current_line.push(Span::styled(text, style));
                }
                MarkdownElement::InlineCode(code) => {
                    current_line.push(Span::styled(
                        format!("`{}`", code),
                        self.config.inline_code_style,
                    ));
                }
                MarkdownElement::CodeBlock { content, language } => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }

                    if self.config.code_block_panel {
                        let title = if language.is_empty() {
                            "code".to_string()
                        } else {
                            language
                        };
                        let panel = Panel::new(Text::plain(content.trim().to_string()))
                            .title(&title)
                            .border_style(BorderStyle::Rounded)
                            .style(self.config.code_style);
                        segments.extend(panel.render(context));
                    } else {
                        for line in content.lines() {
                            segments.push(Segment::line(vec![
                                Span::styled("  ", Style::new()),
                                Span::styled(line.to_string(), self.config.code_style),
                            ]));
                        }
                    }
                    segments.push(Segment::line(vec![]));
                }
                MarkdownElement::StartLink(_url) => {}
                MarkdownElement::EndLink => {}
                MarkdownElement::ListItem { depth, prefix } => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                    let indent = "  ".repeat(depth.saturating_sub(1));
                    current_line.push(Span::raw(format!("{}{}", indent, prefix)));
                }
                MarkdownElement::StartBlockQuote => {
                    current_line.push(Span::styled("│ ", self.config.quote_style));
                }
                MarkdownElement::EndBlockQuote => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                }
                MarkdownElement::SoftBreak => {
                    current_line.push(Span::raw(" "));
                }
                MarkdownElement::HardBreak => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                }
                MarkdownElement::HorizontalRule => {
                    if !current_line.is_empty() {
                        segments.push(Segment::line(std::mem::take(&mut current_line)));
                    }
                    segments.extend(Rule::line().render(context));
                }
            }
        }

        if !current_line.is_empty() {
            segments.push(Segment::line(current_line));
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_basic() {
        let md = Markdown::new("# Hello\n\nWorld");
        let context = RenderContext { width: 40 };
        let segments = md.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_markdown_emphasis() {
        let md = Markdown::new("*italic* and **bold**");
        let context = RenderContext { width: 40 };
        let segments = md.render(&context);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_markdown_code_block() {
        let md = Markdown::new("```rust\nlet x = 42;\n```");
        let context = RenderContext { width: 40 };
        let segments = md.render(&context);

        assert!(!segments.is_empty());
    }
}
