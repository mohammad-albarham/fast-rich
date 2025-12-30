//! Table example demonstrating the Table component.

use rich_rust::prelude::*;

fn main() {
    let console = Console::new();

    console.println("[bold cyan]═══ Basic Table ═══[/]");
    console.println("");

    // Basic table
    let mut table = Table::new();
    table.add_column("Name");
    table.add_column("Age");
    table.add_column("City");
    table.add_row_strs(&["Alice", "30", "New York"]);
    table.add_row_strs(&["Bob", "25", "San Francisco"]);
    table.add_row_strs(&["Charlie", "35", "Chicago"]);

    console.print_renderable(&table);

    console.println("");
    console.println("[bold cyan]═══ Styled Table ═══[/]");
    console.println("");

    // Table with styled columns
    let mut styled_table = Table::new();
    styled_table.add_column(Column::new("ID").right());
    styled_table.add_column(Column::new("Product"));
    styled_table.add_column(Column::new("Price").right());
    styled_table.add_column(Column::new("Status").center());

    styled_table.add_row_strs(&["1", "Widget Pro", "$99.99", "✓ In Stock"]);
    styled_table.add_row_strs(&["2", "Gadget Plus", "$149.99", "✓ In Stock"]);
    styled_table.add_row_strs(&["3", "Thingamajig", "$29.99", "✗ Out of Stock"]);
    styled_table.add_row_strs(&["4", "Doohickey", "$49.99", "✓ In Stock"]);

    console.print_renderable(&styled_table);

    console.println("");
    console.println("[bold cyan]═══ Different Border Styles ═══[/]");
    console.println("");

    // Square borders
    console.println("[dim]Square borders:[/]");
    let mut square_table = Table::new().border_style(BorderStyle::Square);
    square_table.add_column("A");
    square_table.add_column("B");
    square_table.add_row_strs(&["1", "2"]);
    square_table.add_row_strs(&["3", "4"]);
    console.print_renderable(&square_table);

    console.println("");
    console.println("[dim]Heavy borders:[/]");
    let mut heavy_table = Table::new().border_style(BorderStyle::Heavy);
    heavy_table.add_column("A");
    heavy_table.add_column("B");
    heavy_table.add_row_strs(&["1", "2"]);
    heavy_table.add_row_strs(&["3", "4"]);
    console.print_renderable(&heavy_table);

    console.println("");
    console.println("[dim]Double borders:[/]");
    let mut double_table = Table::new().border_style(BorderStyle::Double);
    double_table.add_column("A");
    double_table.add_column("B");
    double_table.add_row_strs(&["1", "2"]);
    double_table.add_row_strs(&["3", "4"]);
    console.print_renderable(&double_table);

    console.println("");
    console.println("[dim]ASCII borders:[/]");
    let mut ascii_table = Table::new().border_style(BorderStyle::Ascii);
    ascii_table.add_column("A");
    ascii_table.add_column("B");
    ascii_table.add_row_strs(&["1", "2"]);
    ascii_table.add_row_strs(&["3", "4"]);
    console.print_renderable(&ascii_table);

    console.println("");
    console.println("[bold cyan]═══ Data Report ═══[/]");
    console.println("");

    // More complex table
    let mut report = Table::new().show_row_lines(true);
    report.add_column(Column::new("Metric"));
    report.add_column(Column::new("Q1").right());
    report.add_column(Column::new("Q2").right());
    report.add_column(Column::new("Q3").right());
    report.add_column(Column::new("Q4").right());
    report.add_column(Column::new("Total").right());

    report.add_row_strs(&["Revenue", "$1.2M", "$1.4M", "$1.6M", "$1.8M", "$6.0M"]);
    report.add_row_strs(&["Expenses", "$0.8M", "$0.9M", "$0.9M", "$1.0M", "$3.6M"]);
    report.add_row_strs(&["Profit", "$0.4M", "$0.5M", "$0.7M", "$0.8M", "$2.4M"]);

    console.print_renderable(&report);
}
