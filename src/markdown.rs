//! Markdown rendering for terminal output.
//!
//! This module provides markdown parsing and rendering using pulldown-cmark.
//! It's feature-gated behind the `markdown` feature.

use crate::console::RenderContext;
use crate::panel::BorderStyle;
use crate::renderable::{Renderable, Segment};
use crate::rule::Rule;
use crate::style::{Color, Style};
use crate::syntax::Syntax;
use crate::table::Table;
use crate::text::{Span, Text};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

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
    /// Style for unordered list bullets
    pub list_bullet_style: Style,
    /// Style for ordered list numbers
    pub list_number_style: Style,
    /// Whether to use a panel for code blocks
    pub code_block_panel: bool,
    /// Theme for syntax highlighting
    pub syntax_theme: crate::syntax::Theme,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig {
            code_style: Style::new().foreground(Color::BrightBlack),
            inline_code_style: Style::new().foreground(Color::Cyan).bold(), // More visible inline code
            heading_styles: [
                Style::new().foreground(Color::Magenta).bold().underline(), // H1
                Style::new().foreground(Color::Blue).bold(),                // H2
                Style::new().foreground(Color::Cyan).bold(),                // H3
                Style::new().foreground(Color::Green).bold(),               // H4
                Style::new().foreground(Color::Yellow).bold(),              // H5
                Style::new().bold(),                                        // H6
            ],
            emphasis_style: Style::new().italic(),
            strong_style: Style::new().bold(),
            link_style: Style::new().foreground(Color::Blue).underline(),
            quote_style: Style::new().foreground(Color::Magenta), // Distinctive color for quote border
            list_bullet_style: Style::new().foreground(Color::Yellow).bold(),
            list_number_style: Style::new().foreground(Color::Yellow).bold(),
            code_block_panel: true,
            syntax_theme: crate::syntax::Theme::Monokai,
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

    /// Parse the markdown and return rendering elements.
    fn parse_internal(&self) -> Vec<MarkdownElement> {
        let options = Options::all();
        let parser = Parser::new_ext(&self.source, options);
        let mut elements = Vec::new();
        let mut style_stack: Vec<Style> = Vec::new();
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut code_block_lang = String::new();
        let mut list_depth = 0;
        let mut ordered_list_num: Option<u64> = None;

        let mut in_table = false;
        let mut in_table_head = false;
        let mut current_table_headers: Vec<String> = Vec::new();
        let mut current_table_rows: Vec<Vec<String>> = Vec::new();
        let mut current_row: Vec<String> = Vec::new();
        let mut current_cell_text = String::new();

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
                        if !in_table {
                            elements.push(MarkdownElement::StartParagraph);
                        }
                    }
                    Tag::Emphasis => style_stack.push(self.config.emphasis_style),
                    Tag::Strong => style_stack.push(self.config.strong_style),
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_block_content.clear();
                        code_block_lang = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                            pulldown_cmark::CodeBlockKind::Indented => String::new(),
                        };
                    }
                    Tag::Link { dest_url, .. } => {
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
                            is_ordered: ordered_list_num.is_some(),
                        });
                    }
                    Tag::BlockQuote(_) => {
                        style_stack.push(self.config.quote_style);
                        elements.push(MarkdownElement::StartBlockQuote);
                    }
                    Tag::Table(_) => {
                        in_table = true;
                        current_table_headers.clear();
                        current_table_rows.clear();
                    }
                    Tag::TableHead => {
                        in_table_head = true;
                        current_row.clear();
                    }
                    Tag::TableRow => {
                        current_row.clear();
                    }
                    Tag::TableCell => {
                        current_cell_text.clear();
                    }
                    _ => {}
                },
                Event::End(tag) => match tag {
                    TagEnd::Heading(_) => {
                        style_stack.pop();
                        elements.push(MarkdownElement::EndHeading);
                    }
                    TagEnd::Paragraph => {
                        if !in_table {
                            elements.push(MarkdownElement::EndParagraph);
                        }
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
                    TagEnd::Table => {
                        in_table = false;
                        elements.push(MarkdownElement::Table {
                            headers: std::mem::take(&mut current_table_headers),
                            rows: std::mem::take(&mut current_table_rows),
                        });
                    }
                    TagEnd::TableHead => {
                        in_table_head = false;
                        current_table_headers = std::mem::take(&mut current_row);
                    }
                    TagEnd::TableRow => {
                        if in_table_head {
                            // Should not happen with current pulldown-cmark
                            current_table_headers = std::mem::take(&mut current_row);
                        } else {
                            current_table_rows.push(std::mem::take(&mut current_row));
                        }
                    }
                    TagEnd::TableCell => {
                        current_row.push(std::mem::take(&mut current_cell_text));
                    }
                    _ => {}
                },
                Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else if in_table {
                        current_cell_text.push_str(&text);
                    } else {
                        let style = style_stack
                            .iter()
                            .fold(Style::new(), |acc, s| acc.combine(s));
                        elements.push(MarkdownElement::Text(text.to_string(), style));
                    }
                }
                Event::Code(code) => {
                    if in_table {
                        current_cell_text.push('`');
                        current_cell_text.push_str(&code);
                        current_cell_text.push('`');
                    } else {
                        elements.push(MarkdownElement::InlineCode(code.to_string()));
                    }
                }
                Event::SoftBreak => {
                    if in_table {
                        current_cell_text.push(' ');
                    } else {
                        elements.push(MarkdownElement::SoftBreak);
                    }
                }
                Event::HardBreak => {
                    if in_table {
                        current_cell_text.push('\n');
                    } else {
                        elements.push(MarkdownElement::HardBreak);
                    }
                }
                Event::Rule => {
                    elements.push(MarkdownElement::HorizontalRule);
                }
                _ => {}
            }
        }
        elements
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
    CodeBlock {
        content: String,
        language: String,
    },
    StartLink(String),
    EndLink,
    ListItem {
        depth: usize,
        prefix: String,
        is_ordered: bool,
    },
    StartBlockQuote,
    EndBlockQuote,
    SoftBreak,
    HardBreak,
    HorizontalRule,
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

impl Renderable for Markdown {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut current_line: Vec<Span> = Vec::new();
        let mut _in_heading = false;
        let mut heading_level = 0;
        let mut blockquote_depth = 0;

        for element in self.parse_internal() {
            // Pre-process for blockquotes
            match element {
                MarkdownElement::StartHeading(level) => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    _in_heading = true;
                    heading_level = level;
                    let prefix = "#".repeat(level + 1) + " ";
                    current_line.push(Span::styled(prefix, self.config.heading_styles[level]));
                }
                MarkdownElement::EndHeading => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    // H1 and H2 get underlines
                    let underline_char = if heading_level == 0 {
                        Some("═")
                    } else if heading_level == 1 {
                        Some("─")
                    } else {
                        None
                    };

                    if let Some(char) = underline_char {
                        let width = if heading_level == 0 {
                            context.width.min(60)
                        } else {
                            context.width.min(40)
                        };
                        let style = self.config.heading_styles[heading_level];
                        let mut underline = Vec::new();
                        underline.push(Span::styled(char.repeat(width), style));
                        self.flush_line(&mut segments, &mut underline, blockquote_depth);
                    }

                    _in_heading = false;
                    self.add_blank_line(&mut segments, blockquote_depth);
                }
                MarkdownElement::StartParagraph => {}
                MarkdownElement::EndParagraph => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    self.add_blank_line(&mut segments, blockquote_depth);
                }
                MarkdownElement::Text(text, style) => {
                    current_line.push(Span::styled(text, style));
                }
                MarkdownElement::InlineCode(code) => {
                    current_line.push(Span::styled(
                        format!(" {} ", code),
                        self.config
                            .inline_code_style
                            .background(Color::rgb(60, 60, 60)),
                    ));
                }
                MarkdownElement::CodeBlock { content, language } => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }

                    let syntax = Syntax::new(&content, &language)
                        .theme(self.config.syntax_theme)
                        .panel(self.config.code_block_panel)
                        .line_numbers(true);

                    let syntax_segments = syntax.render(context);

                    if blockquote_depth > 0 {
                        for segment in syntax_segments {
                            let mut new_spans = vec![self.get_quote_marker(blockquote_depth)];
                            new_spans.extend(segment.spans);
                            segments.push(Segment::line(new_spans));
                        }
                    } else {
                        segments.extend(syntax_segments);
                    }

                    self.add_blank_line(&mut segments, blockquote_depth);
                }
                MarkdownElement::StartLink(_url) => {}
                MarkdownElement::EndLink => {}
                MarkdownElement::ListItem {
                    depth,
                    prefix,
                    is_ordered,
                } => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    let indent = "  ".repeat(depth.saturating_sub(1));
                    let style = if is_ordered {
                        self.config.list_number_style
                    } else {
                        self.config.list_bullet_style
                    };

                    current_line.push(Span::raw(indent));
                    current_line.push(Span::styled(prefix, style));
                }
                MarkdownElement::StartBlockQuote => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    blockquote_depth += 1;
                }
                MarkdownElement::EndBlockQuote => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    blockquote_depth -= 1;
                    self.add_blank_line(&mut segments, blockquote_depth);
                }
                MarkdownElement::SoftBreak => {
                    current_line.push(Span::raw(" "));
                }
                MarkdownElement::HardBreak => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                }
                MarkdownElement::HorizontalRule => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }
                    let rule = Rule::line().style(Style::new().foreground(Color::Yellow));
                    let rule_segments = rule.render(context);
                    if blockquote_depth > 0 {
                        for segment in rule_segments {
                            let mut new_spans = vec![self.get_quote_marker(blockquote_depth)];
                            new_spans.extend(segment.spans);
                            segments.push(Segment::line(new_spans));
                        }
                    } else {
                        segments.extend(rule_segments);
                    }
                    self.add_blank_line(&mut segments, blockquote_depth);
                }
                MarkdownElement::Table { headers, rows } => {
                    if !current_line.is_empty() {
                        self.flush_line(&mut segments, &mut current_line, blockquote_depth);
                    }

                    let mut table = Table::new();
                    for header in headers {
                        table.add_column(
                            crate::table::Column::new(&header)
                                .header_style(Style::new().bold().foreground(Color::Cyan)),
                        );
                    }

                    for row in rows {
                        let cells: Vec<Text> = row.into_iter().map(Text::plain).collect();
                        table.add_row(cells);
                    }

                    table = table.border_style(BorderStyle::Rounded);

                    let table_segments = table.render(context);

                    if blockquote_depth > 0 {
                        for segment in table_segments {
                            let mut new_spans = vec![self.get_quote_marker(blockquote_depth)];
                            new_spans.extend(segment.spans);
                            segments.push(Segment::line(new_spans));
                        }
                    } else {
                        segments.extend(table_segments);
                    }
                    self.add_blank_line(&mut segments, blockquote_depth);
                }
            }
        }

        if !current_line.is_empty() {
            self.flush_line(&mut segments, &mut current_line, blockquote_depth);
        }

        segments
    }
}

impl Markdown {
    fn flush_line(
        &self,
        segments: &mut Vec<Segment>,
        current_line: &mut Vec<Span>,
        quote_depth: usize,
    ) {
        let mut spans = Vec::new();
        if quote_depth > 0 {
            spans.push(self.get_quote_marker(quote_depth));
            spans.push(Span::raw(" ")); // Space after marker
        }
        spans.append(current_line);
        segments.push(Segment::line(spans));
    }

    fn add_blank_line(&self, segments: &mut Vec<Segment>, quote_depth: usize) {
        let mut spans = Vec::new();
        if quote_depth > 0 {
            spans.push(self.get_quote_marker(quote_depth));
        }
        segments.push(Segment::line(spans));
    }

    fn get_quote_marker(&self, _depth: usize) -> Span {
        Span::styled("▎", self.config.quote_style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_basic() {
        let md = Markdown::new("# Hello\n\nWorld");
        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = md.render(&context);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_markdown_table() {
        let md = Markdown::new("| Col1 | Col2 |\n|---|---|\n| Val1 | Val2 |");
        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = md.render(&context);
        assert!(!segments.is_empty());
    }

    #[test]
    fn test_markdown_code_block() {
        let md = Markdown::new("```rust\nfn main() {}\n```");
        let context = RenderContext {
            width: 40,
            height: None,
        };
        let segments = md.render(&context);
        assert!(!segments.is_empty());
    }
}
