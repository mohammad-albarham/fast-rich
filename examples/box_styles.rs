//! Every box style (Rich's 19 `rich.box` boxes plus `Hidden`) on a table
//! and a panel.

use fast_rich::console::Console;
use fast_rich::panel::{BorderStyle, Panel};
use fast_rich::table::Table;

fn main() {
    let console = Console::new();
    console.println("[bold cyan]Box Styles Gallery[/]");
    console.println("");

    let styles = vec![
        ("Ascii", BorderStyle::Ascii),
        ("Ascii2", BorderStyle::Ascii2),
        ("AsciiDoubleHead", BorderStyle::AsciiDoubleHead),
        ("Square", BorderStyle::Square),
        ("SquareDoubleHead", BorderStyle::SquareDoubleHead),
        ("Minimal", BorderStyle::Minimal),
        ("MinimalHeavyHead", BorderStyle::MinimalHeavyHead),
        ("MinimalDoubleHead", BorderStyle::MinimalDoubleHead),
        ("Simple", BorderStyle::Simple),
        ("SimpleHead", BorderStyle::SimpleHead),
        ("SimpleHeavy", BorderStyle::SimpleHeavy),
        ("Horizontals", BorderStyle::Horizontals),
        ("Rounded", BorderStyle::Rounded),
        ("Heavy", BorderStyle::Heavy),
        ("HeavyEdge", BorderStyle::HeavyEdge),
        ("HeavyHead", BorderStyle::HeavyHead),
        ("Double", BorderStyle::Double),
        ("DoubleEdge", BorderStyle::DoubleEdge),
        ("Markdown", BorderStyle::Markdown),
        ("Hidden", BorderStyle::Hidden),
    ];

    // A table and a panel drawn with each of Rich's 19 boxes (+ Hidden).
    for (name, style) in &styles {
        console.println(&format!("[bold yellow]{}[/]", name));

        let mut table = Table::new().border_style(*style);
        table.add_column("Header 1");
        table.add_column("Header 2");
        table.add_row_strs(&["Cell 1", "Cell 2"]);
        table.add_row_strs(&["Cell 3", "Cell 4"]);
        console.print_renderable(&table);

        let panel = Panel::new("Panel content")
            .title(name)
            .border_style(*style)
            .width(30);
        console.print_renderable(&panel);
        console.println("");
    }
}
