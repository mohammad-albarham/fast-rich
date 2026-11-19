//! Before/after tour of the Rich `Text` editing API (G-TX1).
//!
//! Run: cargo run --example text_api

use fast_rich::prelude::*;
use fast_rich::text::Overflow;

fn main() {
    let console = Console::new();
    let red_bold = Style::new().foreground(Color::Red).bold();

    // 1. Color part of a string ------------------------------------------
    console.println("[bold]1. Color one word[/]");

    // Before: split the string yourself and push each piece.
    let mut before = Text::new();
    before.push("Status: ");
    before.push_styled("FAILED", red_bold);
    before.push(" (3 errors)");
    console.print_renderable(&before);

    // After: keep the string whole, style a range.
    let mut after = Text::plain("Status: FAILED (3 errors)");
    after.stylize(red_bold, 8, 14);
    console.print_renderable(&after);

    // 2. Highlight every occurrence ---------------------------------------
    console.println("\n[bold]2. Highlight a word everywhere[/]");

    // Before: find matches by hand, then rebuild the spans.
    let line = "error: disk error on sda1, Error code 5";
    let mut before = Text::new();
    let lower = line.to_lowercase();
    let mut last = 0;
    for (i, m) in lower.match_indices("error") {
        before.push(line[last..i].to_string());
        before.push_styled(line[i..i + m.len()].to_string(), red_bold);
        last = i + m.len();
    }
    before.push(line[last..].to_string());
    console.print_renderable(&before);

    // After: one call, returns the count.
    let mut after = Text::plain(line);
    let n = after.highlight_words(["error"], red_bold, false);
    console.print_renderable(&after);
    console.println(&format!("  ({n} matches)"));

    // 3. Fit text into a fixed-width column ------------------------------
    console.println("\n[bold]3. Truncate to 12 cells with an ellipsis[/]");

    // Before: Text could not be cut; you cropped the String and lost styles.
    let name = "very_long_report_name.txt";
    let cut: String = name.chars().take(11).collect::<String>() + "…";
    console.print_renderable(&Text::plain(cut));

    // After: styles survive the cut, wide characters are handled.
    let mut after = Text::plain(name);
    after.stylize(Style::new().foreground(Color::Cyan), 0, 4);
    after.truncate(12, Some(Overflow::Ellipsis), false);
    console.print_renderable(&after);
    let mut cjk = Text::plain("日本語のファイル名.txt");
    cjk.truncate(12, Some(Overflow::Ellipsis), false);
    console.print_renderable(&cjk);

    // 4. Split and join, keeping styles ----------------------------------
    console.println("\n[bold]4. Split on ', ' and join with ' | '[/]");

    // Before: not possible on Text; split the String and restyle each part.
    let mut tags = Text::plain("rust, cli, terminal");
    tags.stylize(Style::new().italic(), 0, 4);
    // After:
    let parts = tags.split(", ", false);
    let joined = Text::styled(" | ", Style::new().dim()).join(&parts);
    console.print_renderable(&joined);

    // 5. Code with indent guides -----------------------------------------
    console.println("\n[bold]5. Indent guides for code[/]");

    let code = "fn main() {\n    if ready {\n        run();\n    }\n}";
    // Before: no support; print as is.
    console.print_renderable(&Text::plain(code));
    // After:
    let guided = Text::plain(code).with_indent_guides(
        None,
        '│',
        Style::new().dim().foreground(Color::Green),
    );
    console.print_renderable(&guided);

    // 6. Fixed-width table cells -----------------------------------------
    console.println("\n[bold]6. Pad / fit lines to exactly 8 cells[/]");
    for line in Text::plain("ok\nwarning!!\n日本").fit(8) {
        let mut cell = line;
        cell.pad(1, '|');
        console.print_renderable(&cell);
    }
}
