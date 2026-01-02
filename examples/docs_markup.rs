//! Minimal markup demo for docs - fits on one screen
use fast_rich::Console;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Markup Demo ═══[/]\n");

    // Basic styles
    console.println("[bold]Bold[/]  [italic]Italic[/]  [underline]Underline[/]  [dim]Dim[/]");

    // Colors
    console.println("[red]Red[/]  [green]Green[/]  [blue]Blue[/]  [yellow]Yellow[/]  [magenta]Magenta[/]");

    // Background + combined
    console.println("[white on red] Alert [/]  [bold green]Success![/]  [italic cyan]Info[/]");

    // Emoji
    console.println("\n:rocket: Launch  :fire: Hot  :sparkles: Magic  :check_mark: Done");
}
