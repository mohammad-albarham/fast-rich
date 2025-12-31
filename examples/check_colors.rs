use rich_rust::Console;

fn main() {
    let console = Console::new().force_color(true);

    // 1. Basic Style
    console.print("[bold red]This should be bold red[/]");
    console.newline();

    // 2. Nested Style
    console.print("[green]This is green [bold]and bold[/] back to green[/]");
    console.newline();

    // 3. Background
    console.print("[white on blue]White on Blue background[/]");
    console.newline();

    println!("\n--- Debug Info ---");
    let output = console.export_text(&rich_rust::text::Text::from("[bold red]Test[/]")); // This removes codes
    println!("Export text (plain): {:?}", output);

    // We can't easily capture stdout of the real console struct here without internal changes or piping,
    // but the user should see colors when running this.
}
