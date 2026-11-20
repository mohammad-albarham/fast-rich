//! Styles example demonstrating the style system.

use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Style Examples ═══[/]");
    console.println("");

    // Creating styles programmatically
    let error_style = Style::new().foreground(Color::Red).bold();

    let warning_style = Style::new().foreground(Color::Yellow).bold();

    let success_style = Style::new().foreground(Color::Green).bold();

    let info_style = Style::new().foreground(Color::Blue);

    // Using Text with spans
    let mut text = fast_rich::text::Text::new();
    text.push_styled("ERROR: ", error_style);
    text.push("Something went wrong\n");
    text.push_styled("WARNING: ", warning_style);
    text.push("This might be a problem\n");
    text.push_styled("SUCCESS: ", success_style);
    text.push("Operation completed\n");
    text.push_styled("INFO: ", info_style);
    text.push("Here is some information\n");

    console.print_renderable(&text);

    console.println("");
    console.println("[bold cyan]═══ Attributes ═══[/]");
    console.println("");

    // All 13 of Rich's attributes. Terminal support for blink2, frame and
    // encircle is rare; underline2 and overline work in most modern ones.
    for name in [
        "bold",
        "dim",
        "italic",
        "underline",
        "blink",
        "blink2",
        "reverse",
        "conceal",
        "strike",
        "underline2",
        "frame",
        "encircle",
        "overline",
    ] {
        console.print(&format!("[{name}]{name}[/] "));
    }
    console.println("");
    // The same with the builder.
    let fancy = Style::new().underline2().overline().foreground(Color::Cyan);
    let mut text = fast_rich::text::Text::new();
    text.push_styled("double underline + overline", fancy);
    console.print_renderable(&text);

    console.println("");
    console.println("[bold cyan]═══ Color Palette ═══[/]");
    console.println("");

    // Named colors
    let colors = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    for color in &colors {
        console.print(&format!("[{} on {}]  {}  [/] ", color, color, color));
    }
    console.println("");

    // Bright colors
    console.println("");
    let bright_colors = [
        "bright_black",
        "bright_red",
        "bright_green",
        "bright_yellow",
        "bright_blue",
        "bright_magenta",
        "bright_cyan",
        "bright_white",
    ];

    for color in &bright_colors {
        let short = color.replace("bright_", "b_");
        console.print(&format!("[{}]{}[/] ", color, short));
    }
    console.println("");

    console.println("");
    console.println("[bold cyan]═══ 256 Colors ═══[/]");
    console.println("");

    // Show some 256 colors
    for i in (16..52).step_by(1) {
        console.print(&format!("[color({}) on color({})]  [/]", i, i));
    }
    console.println("");
    for i in (52..88).step_by(1) {
        console.print(&format!("[color({}) on color({})]  [/]", i, i));
    }
    console.println("");

    console.println("");
    console.println("[bold cyan]═══ RGB Colors ═══[/]");
    console.println("");

    // Gradient effect using RGB
    for i in 0..32 {
        let r = (i * 8).min(255);
        let g = 100;
        let b = 255 - (i * 8).min(255);
        console.print(&format!(
            "[rgb({},{},{}) on rgb({},{},{})]  [/]",
            r, g, b, r, g, b
        ));
    }
    console.println("");
}
