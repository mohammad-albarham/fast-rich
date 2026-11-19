//! The console theme reaches renderables at render time: components look
//! their styles up by Rich's names (`table.header`, `rule.line`,
//! `bar.complete`, ...), containers pass the theme on to their children,
//! and a style set explicitly on a component wins over the theme.

use fast_rich::columns::Columns;
use fast_rich::console::RenderContext;
use fast_rich::log::{LogLevel, LogMessage};
use fast_rich::panel::Panel;
use fast_rich::progress::{BarColumn, PercentageColumn, ProgressColumn, Task};
use fast_rich::renderable::{Renderable, Segment};
use fast_rich::rule::Rule;
use fast_rich::style::{Color, Style};
use fast_rich::table::{Column, Table};
use fast_rich::text::Span;
use fast_rich::theme::Theme;
use fast_rich::tree::Tree;
use fast_rich::{Layout, Padding, Pretty};

fn spans(segments: &[Segment]) -> Vec<&Span> {
    segments.iter().flat_map(|s| s.spans.iter()).collect()
}

/// The style of the first span whose text contains `needle`.
fn style_of(segments: &[Segment], needle: &str) -> Style {
    spans(segments)
        .into_iter()
        .find(|s| s.text.contains(needle))
        .unwrap_or_else(|| panic!("{needle:?} not rendered"))
        .style
}

fn table() -> Table {
    let mut t = Table::new().title("Title").caption("Caption");
    t.add_column("Head");
    t.add_row_strs(&["cell"]);
    t
}

fn themed(styles: &[(&str, &str)]) -> RenderContext {
    RenderContext::new(40).with_theme(Theme::from_styles(styles.iter().copied()))
}

#[test]
fn table_defaults_are_rich_defaults() {
    let segs = table().render(&RenderContext::new(40));
    assert_eq!(style_of(&segs, "Head"), Style::new().bold());
    assert_eq!(style_of(&segs, "Title"), Style::new().italic());
    assert_eq!(style_of(&segs, "Caption"), Style::new().dim().italic());
    assert!(
        style_of(&segs, "╭").is_empty(),
        "no border style by default"
    );
}

#[test]
fn table_names_restyle_through_theme() {
    let ctx = themed(&[
        ("table.header", "magenta"),
        ("table.border", "blue"),
        ("table.title", "bold"),
        ("table.caption", "red"),
        ("table.cell", "green"),
    ]);
    let segs = table().render(&ctx);
    assert_eq!(style_of(&segs, "Head").foreground, Some(Color::Magenta));
    assert!(
        !style_of(&segs, "Head").bold,
        "the theme replaces Rich's bold"
    );
    assert_eq!(style_of(&segs, "╭").foreground, Some(Color::Blue));
    assert!(style_of(&segs, "Title").bold);
    assert_eq!(style_of(&segs, "Caption").foreground, Some(Color::Red));
    assert_eq!(style_of(&segs, "cell").foreground, Some(Color::Green));
}

#[test]
fn explicit_table_styles_win_over_theme() {
    let ctx = themed(&[("table.header", "magenta"), ("table.border", "blue")]);
    let mut t = Table::new()
        .header_style(Style::parse("cyan"))
        .style(Style::parse("red"));
    t.add_column(Column::new("Head").header_style(Style::parse("underline")));
    t.add_row_strs(&["x"]);
    let segs = t.render(&ctx);
    let head = style_of(&segs, "Head");
    assert_eq!(head.foreground, Some(Color::Cyan));
    assert!(head.underline, "column header style layers on top");
    assert_eq!(style_of(&segs, "╭").foreground, Some(Color::Red));
}

#[test]
fn containers_pass_the_theme_on() {
    let ctx = themed(&[("table.header", "magenta")]);
    let check = |r: &dyn Renderable, what: &str| {
        let segs = r.render(&ctx);
        assert_eq!(
            style_of(&segs, "Head").foreground,
            Some(Color::Magenta),
            "{what} dropped the theme"
        );
    };
    check(&Panel::renderable(table()), "Panel");
    check(&Padding::all(table(), 1), "Padding");
    check(
        &Columns::renderables([std::sync::Arc::new(table()) as _]),
        "Columns",
    );
    let mut layout = Layout::new();
    layout.update(table());
    check(&layout, "Layout");
    let mut split = Layout::new();
    let mut left = Layout::new();
    left.update(table());
    split.split_row(vec![left, Layout::new()]);
    check(&split, "Layout split");
    let mut outer = Table::new();
    outer.add_column("Outer");
    outer.add_row_renderables(vec![std::sync::Arc::new(table())]);
    check(&outer, "Table cell");
    check(
        &Tree::new(fast_rich::tree::TreeNode::renderable(table())),
        "Tree",
    );
}

#[test]
fn rule_line_and_text() {
    // Rich's rule.line is bright_green.
    let segs = Rule::new("Hi").render(&RenderContext::new(20));
    assert_eq!(style_of(&segs, "─").foreground, Some(Color::BrightGreen));

    let ctx = themed(&[("rule.line", "red"), ("rule.text", "italic")]);
    let segs = Rule::new("Hi").render(&ctx);
    assert_eq!(style_of(&segs, "─").foreground, Some(Color::Red));
    assert!(style_of(&segs, "Hi").italic);

    let segs = Rule::new("Hi").style(Style::parse("blue")).render(&ctx);
    assert_eq!(style_of(&segs, "─").foreground, Some(Color::Blue));
}

#[test]
fn tree_line() {
    let mut tree = Tree::new("root");
    tree.add("leaf");
    let segs = tree.render(&themed(&[("tree.line", "yellow")]));
    assert_eq!(style_of(&segs, "└──").foreground, Some(Color::Yellow));
}

#[test]
fn progress_bar_defaults_and_theme() {
    let mut task = Task::new(0, "t", Some(10));
    task.completed = 5;
    let bar = BarColumn::new(10);
    let default = bar.render(&task);
    assert_eq!(default[0].style.foreground, Some(Color::rgb(249, 38, 114)));
    assert_eq!(
        default.last().unwrap().style.foreground,
        Some(Color::Ansi256(237))
    );

    let theme = Theme::from_styles([("bar.complete", "cyan"), ("bar.back", "grey50")]);
    let themed = bar.render_themed(&task, None, &theme);
    assert_eq!(themed[0].style.foreground, Some(Color::Cyan));
    assert_eq!(
        themed.last().unwrap().style.foreground,
        Color::parse("grey50")
    );

    let explicit = BarColumn::new(10).complete_style(Style::parse("red"));
    assert_eq!(
        explicit.render_themed(&task, None, &theme)[0]
            .style
            .foreground,
        Some(Color::Red)
    );

    task.completed = 10;
    task.finished = true;
    assert_eq!(
        bar.render(&task)[0].style.foreground,
        Some(Color::rgb(114, 156, 31))
    );
    // Rich's progress.percentage is magenta.
    assert_eq!(
        PercentageColumn::new().render(&task)[0].style.foreground,
        Some(Color::Magenta)
    );
}

#[test]
fn progress_display_uses_context_theme() {
    let progress =
        fast_rich::progress::Progress::new().with_columns(vec![Box::new(BarColumn::new(10))]);
    let id = progress.add_task("t", Some(10));
    progress.update(id, 5);
    let segs = progress.render(&themed(&[("bar.complete", "blue")]));
    assert_eq!(style_of(&segs, "━").foreground, Some(Color::Blue));
}

#[test]
fn log_levels_and_parts() {
    let msg = LogMessage::new("hello")
        .level(LogLevel::Warning)
        .location("src/main.rs", 3);
    let segs = msg.render(&RenderContext::new(80));
    assert_eq!(style_of(&segs, "WARN").foreground, Some(Color::Yellow));
    let time = style_of(&segs, "[");
    assert!(time.dim && time.foreground == Some(Color::Cyan), "log.time");
    assert_eq!(style_of(&segs, "main.rs"), Style::new().dim(), "log.path");

    let ctx = themed(&[
        ("logging.level.warning", "bold magenta"),
        ("log.message", "italic"),
    ]);
    let segs = msg.render(&ctx);
    let level = style_of(&segs, "WARN");
    assert!(level.bold && level.foreground == Some(Color::Magenta));
    assert!(style_of(&segs, "hello").italic);
}

#[test]
fn pretty_uses_console_theme_unless_given_one() {
    let ctx = themed(&[("repr.number", "red")]);
    let segs = Pretty::new(&vec![7]).render(&ctx);
    assert_eq!(style_of(&segs, "7").foreground, Some(Color::Red));

    let own = Theme::from_styles([("repr.number", "blue")]);
    let segs = Pretty::new(&vec![7]).theme(own).render(&ctx);
    assert_eq!(style_of(&segs, "7").foreground, Some(Color::Blue));
}

#[test]
fn console_theme_reaches_print() {
    let console = fast_rich::Console::capture()
        .width(30)
        .force_color(true)
        .theme(Theme::from_styles([("table.header", "magenta")]));
    console.print_renderable(&Panel::renderable(table()));
    // Magenta (palette 5), not Rich's bold header.
    assert!(console.get_captured_output().contains("\x1b[38;5;5mHead"));
}
