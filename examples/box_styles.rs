use fast_rich::console::Console;
use fast_rich::panel::BorderStyle;
use fast_rich::table::Table;

fn main() {
    let console = Console::new();
    console.print("[bold cyan]Box Styles Gallery[/]");
    console.print("");

    let styles = vec![
        ("Ascii", BorderStyle::Ascii),
        ("AsciiDoubleHead", BorderStyle::AsciiDoubleHead),
        ("Square", BorderStyle::Square),
        ("SquareDoubleHead", BorderStyle::SquareDoubleHead),
        ("Minimal", BorderStyle::Minimal),
        ("MinimalHeavyHead", BorderStyle::MinimalHeavyHead),
        ("MinimalDoubleHead", BorderStyle::MinimalDoubleHead),
        ("Horizontals", BorderStyle::Horizontals),
        ("Rounded", BorderStyle::Rounded),
        ("Heavy", BorderStyle::Heavy),
        ("HeavyEdge", BorderStyle::HeavyEdge),
        ("HeavyHead", BorderStyle::HeavyHead),
        ("Double", BorderStyle::Double),
        ("DoubleEdge", BorderStyle::DoubleEdge),
        ("Hidden", BorderStyle::Hidden),
    ];

    let mut grid = Table::new();
    grid.add_column("Style Name");
    grid.add_column("Panel Example");
    grid.add_column("Table Example");
    grid.border_style(BorderStyle::Rounded);

    // Print separate tables for each style
    for (name, style) in &styles {
        console.print(&format!("[bold yellow]{}[/]", name));

        let mut table = Table::new().border_style(*style).set_title(name);

        table.add_column("Header 1");
        table.add_column("Header 2");
        table.add_row_strs(&["Cell 1", "Cell 2"]);
        table.add_row_strs(&["Cell 3", "Cell 4"]);

        console.print_renderable(&table);
        console.print("");
    }
}
