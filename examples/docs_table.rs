//! Minimal table demo for docs - fits on one screen
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Table Demo ═══[/]\n");

    let mut table = Table::new();
    table.add_column(Column::new("Name"));
    table.add_column(Column::new("Role"));
    table.add_column(Column::new("Status").center());

    table.add_row_strs(&["Alice", "Developer", "✓ Active"]);
    table.add_row_strs(&["Bob", "Designer", "✓ Active"]);
    table.add_row_strs(&["Charlie", "Manager", "Away"]);

    console.print_renderable(&table);
}
