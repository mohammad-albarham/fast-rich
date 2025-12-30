//! Hello world example demonstrating basic rich-rust usage.

use rich_rust::prelude::*;

fn main() {
    let console = Console::new();

    // Simple styled output
    console.println("[bold green]Hello[/], [bold magenta]World[/]!");

    // Use print with multiple styles
    console.println("");
    console.println("[bold]Bold[/], [italic]italic[/], [underline]underline[/], [strikethrough]strikethrough[/]");

    // Colors
    console.println("");
    console.println("[red]Red[/] [green]Green[/] [blue]Blue[/] [yellow]Yellow[/] [magenta]Magenta[/] [cyan]Cyan[/]");

    // Background colors
    console.println("");
    console.println(
        "[white on red] Error [/] [black on yellow] Warning [/] [white on green] Success [/]",
    );

    // Emoji support
    console.println("");
    console.println(":rocket: rich-rust is ready to go! :sparkles:");

    // RGB colors
    console.println("");
    console.println("[#ff6b6b]Coral[/] [#4ecdc4]Teal[/] [#ffe66d]Yellow[/] [#95e1d3]Mint[/]");

    // Nested styles
    console.println("");
    console.println("[bold]This is [red]bold red[/red] and this is [blue]bold blue[/blue][/bold]");
}
