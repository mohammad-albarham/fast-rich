//! Markup example demonstrating the BBCode-style markup parser.

use rich_rust::Console;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Markup Examples ═══[/]");
    console.println("");

    // Basic markup
    console.println("[bold]This is bold text[/]");
    console.println("[italic]This is italic text[/]");
    console.println("[underline]This is underlined text[/]");
    console.println("[strikethrough]This is strikethrough text[/]");
    console.println("[dim]This is dim text[/]");

    console.println("");
    console.println("[bold cyan]═══ Color Markup ═══[/]");
    console.println("");

    // Color markup
    console.println("[red]Red text[/]");
    console.println("[green]Green text[/]");
    console.println("[blue]Blue text[/]");
    console.println("[yellow]Yellow text[/]");

    console.println("");
    console.println("[bold cyan]═══ Combined Styles ═══[/]");
    console.println("");

    // Combined styles
    console.println("[bold red]Bold and red[/]");
    console.println("[italic green]Italic and green[/]");
    console.println("[bold italic underline blue]All the things![/]");

    console.println("");
    console.println("[bold cyan]═══ Background Colors ═══[/]");
    console.println("");

    // Background colors
    console.println("[white on red] White on red [/]");
    console.println("[black on yellow] Black on yellow [/]");
    console.println("[white on blue] White on blue [/]");
    console.println("[black on green] Black on green [/]");

    console.println("");
    console.println("[bold cyan]═══ Nested Markup ═══[/]");
    console.println("");

    // Nested markup
    console.println("[bold]This is bold [red]and this is bold red[/] back to bold[/]");
    console.println("Normal [italic]italic [bold]bold italic[/] just italic[/] normal");

    console.println("");
    console.println("[bold cyan]═══ Emoji Support ═══[/]");
    console.println("");

    // Emoji
    console.println(":smile: Smile");
    console.println(":thumbs_up: Thumbs up");
    console.println(":rocket: Rocket");
    console.println(":fire: Fire");
    console.println(":heart: Heart");
    console.println(":star: Star");
    console.println(":check_mark: Check mark");
    console.println(":warning: Warning");

    console.println("");
    console.println("[bold cyan]═══ Escaped Brackets ═══[/]");
    console.println("");

    // Escaped brackets
    console.println("Use [[bold]] for markup tags");
    console.println("Escape closing too: [[/]]");

    console.println("");
    console.println("[bold cyan]═══ Mixed Content ═══[/]");
    console.println("");

    // Mixed content
    console.println(":rocket: [bold green]Launching[/] the application... :sparkles:");
    console.println(":warning: [bold yellow]Warning:[/] This is a [italic]test[/] message");
    console.println(":check_mark: [bold green]Success![/] All tests passed :tada:");
}
