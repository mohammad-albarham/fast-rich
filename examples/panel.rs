//! Panel example demonstrating panels and rules.

use rich_rust::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Panel Examples ═══[/]");
    console.println("");

    // Basic panel
    let panel = Panel::new("This is a basic panel with some content.").title("Basic Panel");
    console.print_renderable(&panel);

    console.println("");

    // Panel with subtitle
    let panel = Panel::new("Panels can have both titles and subtitles for additional context.")
        .title("With Subtitle")
        .subtitle("v1.0.0");
    console.print_renderable(&panel);

    console.println("");
    console.println("[bold cyan]═══ Border Styles ═══[/]");
    console.println("");

    // Different border styles
    let styles = [
        (BorderStyle::Rounded, "Rounded (default)"),
        (BorderStyle::Square, "Square"),
        (BorderStyle::Heavy, "Heavy"),
        (BorderStyle::Double, "Double"),
        (BorderStyle::Ascii, "ASCII"),
    ];

    for (style, name) in styles {
        let panel = Panel::new("Content here").title(name).border_style(style);
        console.print_renderable(&panel);
        console.println("");
    }

    console.println("[bold cyan]═══ Styled Panel ═══[/]");
    console.println("");

    // Styled panel
    let panel = Panel::new("[italic]This panel has styled borders and custom colors.[/]")
        .title(":warning: Warning")
        .style(Style::new().foreground(Color::Yellow))
        .title_style(Style::new().foreground(Color::Yellow).bold());
    console.print_renderable(&panel);

    console.println("");

    let panel = Panel::new("[italic]This panel indicates an error condition.[/]")
        .title(":x: Error")
        .border_style(BorderStyle::Heavy)
        .style(Style::new().foreground(Color::Red))
        .title_style(Style::new().foreground(Color::Red).bold());
    console.print_renderable(&panel);

    console.println("");

    let panel = Panel::new("[italic]This panel indicates success![/]")
        .title(":check_mark: Success")
        .style(Style::new().foreground(Color::Green))
        .title_style(Style::new().foreground(Color::Green).bold());
    console.print_renderable(&panel);

    console.println("");
    console.println("[bold cyan]═══ Rules ═══[/]");
    console.println("");

    // Rules (horizontal lines)
    console.print_renderable(&Rule::line());
    console.println("");

    console.print_renderable(&Rule::new("Section Title"));
    console.println("");

    // Styled rule
    let rule = Rule::new("Styled Rule")
        .style(Style::new().foreground(Color::Magenta))
        .title_style(Style::new().foreground(Color::Magenta).bold());
    console.print_renderable(&rule);
    console.println("");

    // Different characters
    let rule = Rule::new("Custom Character").character('━');
    console.print_renderable(&rule);
    console.println("");

    let rule = Rule::new("Double Line").character('═');
    console.print_renderable(&rule);
    console.println("");
}
