//! Markdown rendering for terminal output.
//!
//! Parsing uses pulldown-cmark (CommonMark + tables + strikethrough);
//! the layout is a port of Rich's `rich.markdown`: centred `h1`, ` • ` and
//! right-aligned numbers for lists, `▌ ` for quotes, code blocks highlighted
//! with one cell of background padding, tables in Rich's `SIMPLE` box, and
//! Rich's spacing between blocks. The default styles are Rich's.
//!
//! It's feature-gated behind the `markdown` feature.

use crate::console::RenderContext;
use crate::panel::{BorderStyle, Panel};
use crate::renderable::{render_lines_with, Renderable, Segment};
use crate::rule::Rule;
use crate::style::Style;
use crate::syntax::Syntax;
use crate::table::{Cell, Column, Row, Table};
use crate::text::{Alignment, Span, Text};
use crate::theme::Theme;
use pulldown_cmark::{Alignment as MdAlign, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::sync::Arc;

/// Markdown rendering configuration. Defaults are Rich's markdown styles.
///
/// Without an explicit [`Markdown::config`], the styles are taken from the
/// console theme by Rich's names (`markdown.h1`, `markdown.code`, ...; see
/// [`MarkdownConfig::from_theme`]). An explicit config is used as is.
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// Fallback style for code blocks (used when no syntax colours apply)
    pub code_style: Style,
    /// Style for inline code
    pub inline_code_style: Style,
    /// Style for headings h1..h6
    pub heading_styles: [Style; 6],
    /// Style for emphasis (italic)
    pub emphasis_style: Style,
    /// Style for strong (bold)
    pub strong_style: Style,
    /// Style for strikethrough
    pub strike_style: Style,
    /// Style for links (Rich's `markdown.link_url`: the link text when
    /// hyperlinks are on, the `(url)` when they are off)
    pub link_style: Style,
    /// Style for link text when hyperlinks are off (Rich's `markdown.link`)
    pub link_text_style: Style,
    /// Style for blockquotes (text and the `▌` bar)
    pub quote_style: Style,
    /// Style for unordered list bullets
    pub list_bullet_style: Style,
    /// Style for ordered list numbers
    pub list_number_style: Style,
    /// Style for horizontal rules
    pub hr_style: Style,
    /// Style for table borders
    pub table_border_style: Style,
    /// Style for table header cells
    pub table_header_style: Style,
    /// Draw code blocks inside a panel (Rich doesn't; off by default)
    pub code_block_panel: bool,
    /// Theme for syntax highlighting
    pub syntax_theme: crate::syntax::Theme,
    /// Make links clickable (OSC 8). When false, links print as
    /// `text (url)` like Rich's `hyperlinks=False`.
    pub hyperlinks: bool,
    /// Paragraph alignment
    pub justify: Alignment,
}

impl Default for MarkdownConfig {
    /// Rich's default markdown styles.
    fn default() -> Self {
        MarkdownConfig::from_theme(&crate::theme::Theme::new())
    }
}

impl MarkdownConfig {
    /// Styles from `theme`, by Rich's names: `markdown.code_block`,
    /// `markdown.code`, `markdown.h1`..`markdown.h6`, `markdown.em`,
    /// `markdown.strong`, `markdown.s`, `markdown.link_url`,
    /// `markdown.link`, `markdown.block_quote`, `markdown.item.bullet`,
    /// `markdown.item.number`, `markdown.hr`, `markdown.table.border` and
    /// `markdown.table.header`.
    ///
    /// ```
    /// use fast_rich::markdown::MarkdownConfig;
    /// use fast_rich::style::Color;
    /// use fast_rich::theme::Theme;
    /// let theme = Theme::from_styles([("markdown.h2", "bold green")]);
    /// let config = MarkdownConfig::from_theme(&theme);
    /// assert_eq!(config.heading_styles[1].foreground, Some(Color::Green));
    /// assert!(config.heading_styles[0].underline); // Rich's markdown.h1
    /// ```
    pub fn from_theme(theme: &crate::theme::Theme) -> Self {
        let s = |name: &str| theme.style(name);
        MarkdownConfig {
            code_style: s("markdown.code_block"),
            inline_code_style: s("markdown.code"),
            heading_styles: [
                s("markdown.h1"),
                s("markdown.h2"),
                s("markdown.h3"),
                s("markdown.h4"),
                s("markdown.h5"),
                s("markdown.h6"),
            ],
            emphasis_style: s("markdown.em"),
            strong_style: s("markdown.strong"),
            strike_style: s("markdown.s"),
            link_style: s("markdown.link_url"),
            link_text_style: s("markdown.link"),
            quote_style: s("markdown.block_quote"),
            list_bullet_style: s("markdown.item.bullet"),
            list_number_style: s("markdown.item.number"),
            hr_style: s("markdown.hr"),
            table_border_style: s("markdown.table.border"),
            table_header_style: s("markdown.table.header"),
            code_block_panel: false,
            syntax_theme: crate::syntax::Theme::Monokai,
            hyperlinks: true,
            justify: Alignment::Left,
        }
    }
}

/// Parsed blocks and the theme they were styled with (`None` for an
/// explicit config).
type CachedBlocks = (Option<Arc<Theme>>, Vec<Block>);

/// A parsed block element.
#[derive(Debug, Clone)]
enum Block {
    Paragraph(Text),
    Heading(usize, Text),
    Code {
        lang: String,
        code: String,
    },
    Quote(Vec<Block>),
    List {
        start: Option<u64>,
        items: Vec<Vec<Block>>,
    },
    Table {
        aligns: Vec<MdAlign>,
        header: Vec<Text>,
        rows: Vec<Vec<Text>>,
    },
    Rule,
}

impl Block {
    /// Rich's `MarkdownElement.new_line`: a blank line follows the element.
    fn new_line(&self) -> bool {
        !matches!(self, Block::Rule)
    }

    /// Whether the element has child elements; closing those sets Rich's
    /// `new_line`, so such blocks are always preceded by a blank line.
    fn has_child_elements(&self) -> bool {
        matches!(
            self,
            Block::Quote(_) | Block::List { .. } | Block::Table { .. }
        )
    }
}

/// Rendered markdown content.
///
/// The parsed block tree is cached after the first render and reused until
/// the configuration changes, so re-rendering inside a `Live` display
/// doesn't re-parse the document every frame.
#[derive(Debug)]
pub struct Markdown {
    source: String,
    /// Explicit config; `None` takes the styles from the render theme.
    config: Option<MarkdownConfig>,
    /// Parsed blocks and the theme they were styled with (`None` for an
    /// explicit config).
    cache: std::sync::Mutex<Option<CachedBlocks>>,
}

impl Clone for Markdown {
    fn clone(&self) -> Self {
        Markdown {
            source: self.source.clone(),
            config: self.config.clone(),
            cache: std::sync::Mutex::new(self.cache.lock().ok().and_then(|g| g.clone())),
        }
    }
}

/// Containers open while parsing.
enum Open {
    Quote(Vec<Block>),
    List(Option<u64>, Vec<Vec<Block>>),
    Item(Vec<Block>),
}

/// A table being collected.
#[derive(Default)]
struct TableState {
    aligns: Vec<MdAlign>,
    header: Vec<Text>,
    rows: Vec<Vec<Text>>,
    row: Vec<Text>,
}

/// Inline text being collected, with the active inline styles.
struct Inline {
    text: Text,
    styles: Vec<Style>,
    links: Vec<Option<String>>,
    /// Links with hyperlinks off: (text start, url) to append "(url)".
    plain_links: Vec<(usize, String)>,
}

impl Inline {
    fn new(base: Style) -> Self {
        Inline {
            text: Text::new(),
            styles: vec![base],
            links: Vec::new(),
            plain_links: Vec::new(),
        }
    }

    fn style(&self) -> Style {
        self.styles
            .iter()
            .fold(Style::new(), |acc, s| acc.combine(s))
    }

    fn push(&mut self, s: &str) {
        let style = self.style();
        let link = self.links.iter().rev().find_map(Clone::clone);
        let span = match link {
            Some(url) => Span::linked(s.to_string(), style, url),
            None => Span::styled(s.to_string(), style),
        };
        self.text.push_span(span);
    }
}

impl Markdown {
    /// Create a new Markdown from source text.
    pub fn new(source: &str) -> Self {
        Markdown {
            source: source.to_string(),
            config: None,
            cache: std::sync::Mutex::new(None),
        }
    }

    /// Set the rendering configuration. It is used as is: theme styles
    /// no longer apply (start from [`MarkdownConfig::from_theme`] to keep
    /// them).
    pub fn config(mut self, config: MarkdownConfig) -> Self {
        self.config = Some(config);
        self.cache = std::sync::Mutex::new(None);
        self
    }

    /// The config to render with: the explicit one, or one from `theme`.
    fn resolved_config(&self, theme: &Theme) -> MarkdownConfig {
        match &self.config {
            Some(config) => config.clone(),
            None => MarkdownConfig::from_theme(theme),
        }
    }

    fn blocks(&self, cfg: &MarkdownConfig, theme: &Arc<Theme>) -> Vec<Block> {
        let key = self.config.is_none().then(|| theme.clone());
        let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        let fresh = match (&*cache, &key) {
            (Some((None, _)), None) => true,
            (Some((Some(cached), _)), Some(theme)) => Arc::ptr_eq(cached, theme),
            _ => false,
        };
        if !fresh {
            *cache = Some((key, self.parse(cfg)));
        }
        cache.as_ref().map(|(_, b)| b.clone()).unwrap_or_default()
    }

    /// Append a finished block to the innermost open container.
    fn push_block(root: &mut Vec<Block>, open: &mut [Open], block: Block) {
        match open.last_mut() {
            Some(Open::Quote(blocks)) | Some(Open::Item(blocks)) => blocks.push(block),
            // Blocks directly in a list (shouldn't happen) go to the root.
            Some(Open::List(..)) | None => root.push(block),
        }
    }

    fn parse(&self, cfg: &MarkdownConfig) -> Vec<Block> {
        let options =
            Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
        let mut root: Vec<Block> = Vec::new();
        let mut open: Vec<Open> = Vec::new();
        let mut inline: Option<Inline> = None;
        let mut heading: Option<usize> = None;
        let mut code: Option<(String, String)> = None;
        let mut table: Option<TableState> = None;
        // Rich renders an image ("🌆 <alt> ") before the paragraph it sits
        // in. `image` collects the current image's (file name, alt text).
        let mut images = String::new();
        let mut image: Option<(String, String)> = None;

        // Tight list items have text without a Paragraph event.
        let ensure_inline = |inline: &mut Option<Inline>| {
            if inline.is_none() {
                *inline = Some(Inline::new(Style::new()));
            }
        };
        let finish_paragraph = |inline: &mut Option<Inline>,
                                images: &mut String,
                                root: &mut Vec<Block>,
                                open: &mut Vec<Open>| {
            if let Some(mut done) = inline.take() {
                if !images.is_empty() {
                    let mut text = Text::plain(std::mem::take(images));
                    text.spans.append(&mut done.text.spans);
                    done.text = text;
                }
                Self::push_block(root, open, Block::Paragraph(done.text));
            }
        };

        for event in Parser::new_ext(&self.source, options) {
            if let Some((_, buf)) = code.as_mut() {
                match event {
                    Event::Text(t) => {
                        buf.push_str(&t);
                        continue;
                    }
                    Event::End(TagEnd::CodeBlock) => {}
                    _ => continue,
                }
            }
            match event {
                Event::Start(Tag::Paragraph) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    inline = Some(Inline::new(Style::new()));
                }
                Event::End(TagEnd::Paragraph) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                }
                Event::Start(Tag::Heading { level, .. }) => {
                    let n = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    heading = Some(n);
                    inline = Some(Inline::new(cfg.heading_styles[n - 1]));
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let (Some(n), Some(done)) = (heading.take(), inline.take()) {
                        Self::push_block(&mut root, &mut open, Block::Heading(n, done.text));
                    }
                }
                Event::Start(Tag::CodeBlock(kind)) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    let lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(info) => {
                            info.split_whitespace().next().unwrap_or("").to_string()
                        }
                        pulldown_cmark::CodeBlockKind::Indented => String::new(),
                    };
                    code = Some((lang, String::new()));
                }
                Event::End(TagEnd::CodeBlock) => {
                    if let Some((lang, buf)) = code.take() {
                        Self::push_block(&mut root, &mut open, Block::Code { lang, code: buf });
                    }
                }
                Event::Start(Tag::BlockQuote(_)) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    open.push(Open::Quote(Vec::new()));
                }
                Event::End(TagEnd::BlockQuote(_)) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    if let Some(Open::Quote(blocks)) = open.pop() {
                        Self::push_block(&mut root, &mut open, Block::Quote(blocks));
                    }
                }
                Event::Start(Tag::List(start)) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    open.push(Open::List(start, Vec::new()));
                }
                Event::End(TagEnd::List(_)) => {
                    if let Some(Open::List(start, items)) = open.pop() {
                        Self::push_block(&mut root, &mut open, Block::List { start, items });
                    }
                }
                Event::Start(Tag::Item) => {
                    open.push(Open::Item(Vec::new()));
                }
                Event::End(TagEnd::Item) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    if let Some(Open::Item(blocks)) = open.pop() {
                        if let Some(Open::List(_, items)) = open.last_mut() {
                            items.push(blocks);
                        }
                    }
                }
                Event::Start(Tag::Table(aligns)) => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    table = Some(TableState {
                        aligns,
                        ..TableState::default()
                    });
                }
                Event::End(TagEnd::TableHead) => {
                    if let Some(t) = table.as_mut() {
                        t.header = std::mem::take(&mut t.row);
                    }
                }
                Event::Start(Tag::TableRow) => {}
                Event::End(TagEnd::TableRow) => {
                    if let Some(t) = table.as_mut() {
                        let row = std::mem::take(&mut t.row);
                        t.rows.push(row);
                    }
                }
                Event::Start(Tag::TableCell) => {
                    inline = Some(Inline::new(Style::new()));
                }
                Event::End(TagEnd::TableCell) => {
                    if let (Some(t), Some(done)) = (table.as_mut(), inline.take()) {
                        t.row.push(done.text);
                    }
                }
                Event::End(TagEnd::Table) => {
                    if let Some(TableState {
                        aligns,
                        header,
                        rows,
                        ..
                    }) = table.take()
                    {
                        Self::push_block(
                            &mut root,
                            &mut open,
                            Block::Table {
                                aligns,
                                header,
                                rows,
                            },
                        );
                    }
                }
                Event::Rule => {
                    finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
                    Self::push_block(&mut root, &mut open, Block::Rule);
                }
                Event::Start(Tag::Emphasis) => {
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.styles.push(cfg.emphasis_style);
                    }
                }
                Event::Start(Tag::Strong) => {
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.styles.push(cfg.strong_style);
                    }
                }
                Event::Start(Tag::Strikethrough) => {
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.styles.push(cfg.strike_style);
                    }
                }
                Event::End(TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough) => {
                    if let Some(i) = inline.as_mut() {
                        i.styles.pop();
                    }
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        if cfg.hyperlinks {
                            i.styles.push(cfg.link_style);
                            i.links.push(Some(dest_url.to_string()));
                        } else {
                            i.styles.push(cfg.link_text_style);
                            i.links.push(None);
                            let start = i.text.spans.len();
                            i.plain_links.push((start, dest_url.to_string()));
                        }
                    }
                }
                Event::End(TagEnd::Link) => {
                    if let Some(i) = inline.as_mut() {
                        i.styles.pop();
                        i.links.pop();
                        if let Some((_, url)) = i.plain_links.pop() {
                            i.push(" (");
                            i.styles.push(cfg.link_style);
                            i.push(&url);
                            i.styles.pop();
                            i.push(")");
                        }
                    }
                }
                Event::Start(Tag::Image { dest_url, .. }) => {
                    let name = dest_url
                        .trim_end_matches('/')
                        .rsplit('/')
                        .next()
                        .unwrap_or("")
                        .to_string();
                    image = Some((name, String::new()));
                }
                Event::End(TagEnd::Image) => {
                    if let Some((name, alt)) = image.take() {
                        images.push_str("🌆 ");
                        images.push_str(if alt.is_empty() { &name } else { &alt });
                        images.push(' ');
                    }
                }
                Event::Text(t) => {
                    if let Some((_, alt)) = image.as_mut() {
                        alt.push_str(&t);
                        continue;
                    }
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.push(&t);
                    }
                }
                Event::Code(t) => {
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.styles.push(cfg.inline_code_style);
                        i.push(&t);
                        i.styles.pop();
                    }
                }
                Event::SoftBreak => {
                    if let Some(i) = inline.as_mut() {
                        i.push(" ");
                    }
                }
                Event::HardBreak => {
                    if let Some(i) = inline.as_mut() {
                        i.push("\n");
                    }
                }
                Event::TaskListMarker(done) => {
                    // CommonMark (Rich) has no task lists: the brackets stay.
                    ensure_inline(&mut inline);
                    if let Some(i) = inline.as_mut() {
                        i.push(if done { "[x] " } else { "[ ] " });
                    }
                }
                _ => {}
            }
        }
        finish_paragraph(&mut inline, &mut images, &mut root, &mut open);
        root
    }

    /// Children of a list item or quote: no blank lines between them.
    fn render_children(
        &self,
        cfg: &MarkdownConfig,
        blocks: &[Block],
        context: &RenderContext,
    ) -> Vec<Vec<Span>> {
        blocks
            .iter()
            .flat_map(|b| self.render_block(cfg, b, context))
            .collect()
    }

    /// Prefix every line, applying `base` under the content styles.
    fn prefixed(lines: Vec<Vec<Span>>, first: Span, rest: Span, base: Style) -> Vec<Vec<Span>> {
        lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                let mut out = vec![if i == 0 { first.clone() } else { rest.clone() }];
                out.extend(line.into_iter().map(|mut s| {
                    s.style = base.combine(&s.style);
                    s
                }));
                out
            })
            .collect()
    }

    fn render_block(
        &self,
        cfg: &MarkdownConfig,
        block: &Block,
        context: &RenderContext,
    ) -> Vec<Vec<Span>> {
        let width = context.width;
        match block {
            Block::Paragraph(text) => {
                let mut text = text.clone();
                text.alignment = cfg.justify;
                render_lines_with(&text, &context.child(width), Style::new())
            }
            Block::Heading(level, text) => {
                let mut text = text.clone();
                text.alignment = if *level == 1 {
                    Alignment::Center
                } else {
                    Alignment::Left
                };
                render_lines_with(&text, &context.child(width), Style::new())
            }
            Block::Code { lang, code } => self.render_code(cfg, lang, code.trim_end(), context),
            Block::Quote(children) => {
                let lines =
                    self.render_children(cfg, children, &context.child(width.saturating_sub(4)));
                let lines: Vec<Vec<Span>> = lines
                    .into_iter()
                    .map(|l| crate::renderable::fit_line(l, width.saturating_sub(4), Style::new()))
                    .collect();
                let bar = Span::styled("▌ ", cfg.quote_style);
                Self::prefixed(lines, bar.clone(), bar, cfg.quote_style)
            }
            Block::List { start, items } => {
                let mut out = Vec::new();
                match start {
                    None => {
                        for item in items {
                            let lines = self.render_children(
                                cfg,
                                item,
                                &context.child(width.saturating_sub(3)),
                            );
                            out.extend(Self::prefixed(
                                lines,
                                Span::styled(" • ", cfg.list_bullet_style),
                                Span::styled("   ", cfg.list_bullet_style),
                                Style::new(),
                            ));
                        }
                    }
                    Some(first) => {
                        let last = first + items.len() as u64;
                        let number_width = last.to_string().len() + 2;
                        for (i, item) in items.iter().enumerate() {
                            let lines = self.render_children(
                                cfg,
                                item,
                                &context.child(width.saturating_sub(number_width)),
                            );
                            let numeral =
                                format!("{:>w$} ", first + i as u64, w = number_width - 1);
                            out.extend(Self::prefixed(
                                lines,
                                Span::styled(numeral, cfg.list_number_style),
                                Span::styled(" ".repeat(number_width), cfg.list_number_style),
                                Style::new(),
                            ));
                        }
                    }
                }
                out
            }
            Block::Table {
                aligns,
                header,
                rows,
            } => {
                // Rich: Table(box=SIMPLE, pad_edge=False, show_edge=True,
                // collapse_padding=True, style="markdown.table.border").
                let mut table = Table::new()
                    .border_style(BorderStyle::Simple)
                    .style(cfg.table_border_style)
                    // Rich's markdown.table.header is "not bold cyan": it
                    // cancels table.header's bold.
                    .header_style(Style::new())
                    .pad_edge(false)
                    .collapse_padding(true)
                    .show_edge(true);
                for (i, cell) in header.iter().enumerate() {
                    let align = match aligns.get(i) {
                        Some(MdAlign::Center) => Alignment::Center,
                        Some(MdAlign::Right) => Alignment::Right,
                        _ => Alignment::Left,
                    };
                    let mut text = cell.clone();
                    for span in &mut text.spans {
                        span.style = cfg.table_header_style.combine(&span.style);
                    }
                    table.add_column(
                        Column::new("")
                            .header_text(text)
                            .header_style(Style::new())
                            .align(align),
                    );
                }
                for row in rows {
                    table.add_row_obj(Row::cells(row.iter().cloned().map(Cell::Text)));
                }
                render_lines_with(&table, &context.child(width), Style::new())
            }
            Block::Rule => {
                let rule = Rule::line().characters("-").style(cfg.hr_style);
                let mut lines = render_lines_with(&rule, &context.child(width), Style::new());
                lines.push(Vec::new());
                lines
            }
        }
    }

    /// Rich's code block: `Syntax(code, lexer, word_wrap=True, padding=1)`.
    fn render_code(
        &self,
        cfg: &MarkdownConfig,
        lang: &str,
        code: &str,
        context: &RenderContext,
    ) -> Vec<Vec<Span>> {
        let width = context.width;
        let lexer = if lang.is_empty() { "text" } else { lang };
        let syntax = Syntax::new(code, lexer)
            .line_numbers(false)
            .panel(false)
            .theme(cfg.syntax_theme);
        let (lines, background) = syntax.highlighted_lines();
        let bg = background.map_or(cfg.code_style, |c| Style::new().background(c));
        let inner = width.saturating_sub(2);
        let blank = || vec![Span::styled(" ".repeat(width), bg)];
        let mut out = vec![blank()];
        for line in lines {
            // word_wrap=True: long lines wrap inside the padding.
            let text = Text::from_spans(line);
            for wrapped in render_lines_with(&text, &context.child(inner), bg) {
                let mut row = vec![Span::styled(" ", bg)];
                row.extend(wrapped.into_iter().map(|mut s| {
                    s.style = bg.combine(&s.style);
                    s
                }));
                row.push(Span::styled(" ", bg));
                out.push(row);
            }
        }
        out.push(blank());
        if cfg.code_block_panel {
            let mut text = Text::new();
            let count = out.len();
            for (i, line) in out.into_iter().enumerate() {
                for span in line {
                    text.push_span(span);
                }
                if i + 1 < count {
                    text.push("\n");
                }
            }
            let panel = Panel::new(text).title(lexer).padding(0, 0);
            return render_lines_with(&panel, &context.child(width), Style::new());
        }
        out
    }
}

impl Renderable for Markdown {
    /// Rich's spacing: a blank line before a top-level block when the
    /// element closed last had `new_line` set (blocks with child elements
    /// always do; rules don't).
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let cfg = self.resolved_config(&context.theme);
        let mut out: Vec<Segment> = Vec::new();
        let mut new_line = false;
        for block in self.blocks(&cfg, &context.theme) {
            if new_line || block.has_child_elements() {
                out.push(Segment::empty_line());
            }
            out.extend(
                self.render_block(&cfg, &block, context)
                    .into_iter()
                    .map(Segment::line),
            );
            new_line = block.new_line();
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    fn plain(md: &str, width: usize) -> Vec<String> {
        Markdown::new(md)
            .render(&RenderContext::new(width))
            .iter()
            .map(|s| s.plain_text().trim_end().to_string())
            .collect()
    }

    // Expected outputs come from Rich 15 (`rich.markdown.Markdown`).

    #[test]
    fn test_paragraphs_and_headings() {
        assert_eq!(
            plain("para one\n\npara two", 30),
            ["para one", "", "para two"]
        );
        assert_eq!(
            plain("# H1\n## H2\ntext", 30),
            ["              H1", "", "H2", "", "text"]
        );
    }

    #[test]
    fn test_lists() {
        assert_eq!(
            plain("- a\n- b\n\n1. x\n2. y", 30),
            ["", " • a", " • b", "", " 1 x", " 2 y"]
        );
        assert_eq!(
            plain("- a\n\n  second para\n- b", 30),
            ["", " • a", "   second para", " • b"]
        );
        assert_eq!(
            plain("- [ ] todo\n- [x] done", 30),
            ["", " • [ ] todo", " • [x] done"]
        );
    }

    #[test]
    fn test_nested_ordered_list_numbering() {
        assert_eq!(
            plain(
                "1. outer one\n2. outer two\n   1. inner a\n   2. inner b\n3. outer three\n",
                40
            ),
            [
                "",
                " 1 outer one",
                " 2 outer two",
                "    1 inner a",
                "    2 inner b",
                " 3 outer three"
            ]
        );
    }

    #[test]
    fn test_code_quote_rule_table() {
        assert_eq!(
            plain("text\n\n```\ncode\n```\n\nafter", 30),
            ["text", "", "", " code", "", "", "after"]
        );
        assert_eq!(
            plain("> q1\n>\n> q2\n\nafter", 30),
            ["", "▌ q1", "▌ q2", "", "after"]
        );
        assert_eq!(
            plain("---\nafter", 30),
            ["------------------------------", "", "after"]
        );
        assert_eq!(
            plain("| a | b |\n|---|--:|\n| 1 | 22 |\n\nafter", 30),
            ["", "", " a   b", " ─────", " 1  22", "", "", "after"]
        );
    }

    #[test]
    fn test_inline_styles_and_links() {
        let md = Markdown::new("**b** *i* `c` [l](https://x.y)");
        let segs = md.render(&RenderContext::new(40));
        let spans: Vec<&Span> = segs.iter().flat_map(|s| s.spans.iter()).collect();
        let find = |t: &str| spans.iter().find(|s| s.text == t).copied();
        assert!(find("b").is_some_and(|s| s.style.bold));
        assert!(find("i").is_some_and(|s| s.style.italic));
        assert!(find("c").is_some_and(|s| s.style.foreground == Some(Color::Cyan)));
        assert_eq!(
            find("l").and_then(|s| s.link.as_deref()),
            Some("https://x.y")
        );
    }
}
