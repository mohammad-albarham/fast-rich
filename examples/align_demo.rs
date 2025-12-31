use rich_rust::prelude::*;
use rich_rust::align::{Align, VerticalAlignment};

fn main() {
    let console = Console::new().width(60);
    console.rule("[bold red]Alignment Demo[/]");

    let text = Text::plain("Hello, World!").style(Style::new().bold().foreground(Color::Green));

    console.print("[bold]Left Aligned (default):[/]");
    console.print_renderable(&Align::left(text.clone()));
    console.newline();

    console.print("[bold]Center Aligned:[/]");
    console.print_renderable(&Align::center(text.clone()));
    console.newline();

    console.print("[bold]Right Aligned:[/]");
    console.print_renderable(&Align::right(text.clone()));
    console.newline();

    console.print("[bold]Vertical Alignment (Height 5, Middle):[/]");
    let panel = Panel::new(text.clone()).title("Panel");
    
    // Vertical alignment needs a container height or explicit height
    console.print_renderable(
        &Align::center(panel)
        .vertical(VerticalAlignment::Middle)
        .height(5)
    );

    console.rule("[bold red]End Demo[/]");
}
