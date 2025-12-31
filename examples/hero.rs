//! Simple hero example for documentation screenshots
use fast_rich::prelude::*;

fn main() {
    let console = Console::new().force_color(true);

    console.print("[bold cyan]Fast-Rich[/] - Beautiful terminal output for Rust\n");

    // Styled text
    console.print("[bold]Bold[/], [italic]italic[/], [red]colors[/], and [underline]more[/]!\n");

    // Simple table
    let mut table = Table::new();
    table.add_column("Feature");
    table.add_column("Status");
    table.add_row_strs(&["Rich Text", "✅"]);
    table.add_row_strs(&["Tables", "✅"]);
    table.add_row_strs(&["Progress", "✅"]);
    console.print_renderable(&table);
}
