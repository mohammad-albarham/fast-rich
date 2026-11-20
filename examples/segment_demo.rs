//! Rich's Segment toolkit and Control codes (G-SG1).
//!
//! Render something to lines, then crop, pad, shape, align and divide the
//! lines yourself, the way Rich's `Segment` helpers do; finish with a few
//! `Control` escapes.
//!
//! Run: cargo run --example segment_demo

use fast_rich::control::Control;
use fast_rich::prelude::*;
use fast_rich::renderable::Segment;
use fast_rich::text::Span;

fn show(console: &Console, title: &str, lines: &[Vec<Span>]) {
    console.println(&format!("[bold]{title}[/]"));
    for line in lines {
        let mut text = Text::from_spans(line.iter().cloned());
        text.push("|");
        console.print_renderable(&text.no_wrap());
    }
}

fn main() {
    let console = Console::new();
    let spans = vec![
        Span::styled("fast-rich ", Style::parse("bold magenta")),
        Span::styled("segments\n", Style::parse("italic")),
        Span::raw("日本語 wide text\n"),
        Span::styled("last line", Style::parse("underline green")),
    ];

    // Split at newlines and make every line exactly 12 cells.
    let lines = Segment::split_and_crop_lines(&spans, 12, Style::parse("on grey23"), true, false);
    show(&console, "split_and_crop_lines(12)", &lines);

    // Give the block a fixed shape and center it vertically.
    let (width, height) = Segment::get_shape(&lines);
    console.println(&format!("shape: {width}x{height}"));
    let middle = Segment::align_middle(&lines, width, 5, Style::parse("on blue"), false);
    show(&console, "align_middle(height 5)", &middle);

    // Cut one line into columns, splitting the wide characters cleanly.
    let line =
        Segment::adjust_line_length(&[Span::raw("日本語 wide text")], 15, Style::new(), true);
    let parts = Segment::divide(&line, [3, 9, 15]);
    for (i, part) in parts.iter().enumerate() {
        console.println(&format!("divide part {i}: {:?}", Segment::plain(part)));
    }

    // Restyle and simplify.
    let styled = Segment::apply_style(&spans, Style::new(), Style::new().reverse());
    let colorless = Segment::remove_color(&styled);
    console.println(&format!(
        "{} spans -> {} after simplify(strip_styles)",
        colorless.len(),
        Segment::simplify(&Segment::strip_styles(&colorless)).len()
    ));

    // Control codes: printed as raw escapes, taking no cells.
    console.print_renderable(&Control::title("segment_demo"));
    console.print("before");
    console.print_renderable(&Control::move_cursor(-6, 0));
    console.println("AFTER!");
    console.print_renderable(&Control::bell());
}
