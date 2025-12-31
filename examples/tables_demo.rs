use rich_rust::prelude::*;

fn main() {
    let console = Console::new().force_color(true);

    console.rule("[bold cyan]Table Features Demo[/]");
    console.newline();

    // 1. Basic Table
    console.print("[bold]1. Basic Table with Styling[/]");
    let mut table = Table::new().title("Star Wars Movies");
    table.add_column("Episode");
    table.add_column("Title");
    table.add_column("Director");

    table.add_row_strs(&["IV", "A New Hope", "George Lucas"]);
    table.add_row_strs(&["V", "The Empire Strikes Back", "Irvin Kershner"]);
    table.add_row_strs(&["VI", "Return of the Jedi", "Richard Marquand"]);

    console.print_renderable(&table);
    console.newline();

    // 2. Column Attributes (Alignment & Width)
    console.print("[bold]2. Column Alignment & Width[/]");
    let mut table2 = Table::new().title("Server Status");
    table2
        .add_column(
            Column::new("Region")
                .align(ColumnAlign::Left)
                .width(rich_rust::table::ColumnWidth::Fixed(10)),
        )
        .add_column(
            Column::new("Status")
                .align(ColumnAlign::Center)
                .width(rich_rust::table::ColumnWidth::Fixed(10)),
        )
        .add_column(
            Column::new("Uptime")
                .align(ColumnAlign::Right)
                .width(rich_rust::table::ColumnWidth::Fixed(10)),
        );

    table2.add_row(vec![
        rich_rust::text::Text::plain("US-East"),
        rich_rust::markup::parse("[green]Online[/]"),
        rich_rust::text::Text::plain("99.9%"),
    ]);
    table2.add_row(vec![
        rich_rust::text::Text::plain("EU-West"),
        rich_rust::markup::parse("[yellow]Degraded[/]"),
        rich_rust::text::Text::plain("95.0%"),
    ]);
    table2.add_row(vec![
        rich_rust::text::Text::plain("AP-South"),
        rich_rust::markup::parse("[red]Offline[/]"),
        rich_rust::text::Text::plain("0.0%"),
    ]);

    console.print_renderable(&table2);
    console.newline();

    // 3. Border Styles
    console.print("[bold]3. Border Variations[/]");
    let styles = [
        ("Rounded", rich_rust::panel::BorderStyle::Rounded),
        ("Square", rich_rust::panel::BorderStyle::Square),
        ("Heavy", rich_rust::panel::BorderStyle::Heavy),
        ("Double", rich_rust::panel::BorderStyle::Double),
        ("Ascii", rich_rust::panel::BorderStyle::Ascii),
        ("Minimal", rich_rust::panel::BorderStyle::Minimal),
        ("Hidden", rich_rust::panel::BorderStyle::Hidden),
    ];

    for (name, style) in styles {
        let mut t = Table::new().border_style(style);
        t.add_column("Style Name");
        t.add_column("Example");
        t.add_row_strs(&[name, "Content"]);
        console.print_renderable(&t);
    }
    console.newline();

    // 4. Styling Rows and Cells
    console.print("[bold]4. Row & Cell Styling[/]");
    let mut table3 = Table::new().title("Log Entries");
    table3.add_column("Level");
    table3.add_column("Message");

    // Row style applied to whole row content
    table3.add_row_strs(&["INFO", "System started"]);

    // Manual markup in cells
    table3.add_row(vec![
        rich_rust::markup::parse("[yellow]WARN[/]"),
        rich_rust::markup::parse("Memory usage high"),
    ]);

    table3.add_row(vec![
        rich_rust::markup::parse("[red bold]ERROR[/]"),
        rich_rust::markup::parse("[red]Disk full[/]"),
    ]);

    console.print_renderable(&table3);

    console.rule("[bold cyan]End Table Demo[/]");
}
