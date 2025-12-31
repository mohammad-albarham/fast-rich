use fast_rich::prelude::*;

fn run(console: &Console) {
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
                .width(fast_rich::table::ColumnWidth::Fixed(10)),
        )
        .add_column(
            Column::new("Status")
                .align(ColumnAlign::Center)
                .width(fast_rich::table::ColumnWidth::Fixed(10)),
        )
        .add_column(
            Column::new("Uptime")
                .align(ColumnAlign::Right)
                .width(fast_rich::table::ColumnWidth::Fixed(10)),
        );

    table2.add_row(vec![
        fast_rich::text::Text::plain("US-East"),
        fast_rich::markup::parse("[green]Online[/]"),
        fast_rich::text::Text::plain("99.9%"),
    ]);
    table2.add_row(vec![
        fast_rich::text::Text::plain("EU-West"),
        fast_rich::markup::parse("[yellow]Degraded[/]"),
        fast_rich::text::Text::plain("95.0%"),
    ]);
    table2.add_row(vec![
        fast_rich::text::Text::plain("AP-South"),
        fast_rich::markup::parse("[red]Offline[/]"),
        fast_rich::text::Text::plain("0.0%"),
    ]);

    console.print_renderable(&table2);
    console.newline();

    // 3. Border Styles
    console.print("[bold]3. Border Variations[/]");
    let styles = [
        ("Rounded", fast_rich::panel::BorderStyle::Rounded),
        ("Square", fast_rich::panel::BorderStyle::Square),
        ("Heavy", fast_rich::panel::BorderStyle::Heavy),
        ("Double", fast_rich::panel::BorderStyle::Double),
        ("Ascii", fast_rich::panel::BorderStyle::Ascii),
        ("Minimal", fast_rich::panel::BorderStyle::Minimal),
        ("Hidden", fast_rich::panel::BorderStyle::Hidden),
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
        fast_rich::markup::parse("[yellow]WARN[/]"),
        fast_rich::markup::parse("Memory usage high"),
    ]);

    table3.add_row(vec![
        fast_rich::markup::parse("[red bold]ERROR[/]"),
        fast_rich::markup::parse("[red]Disk full[/]"),
    ]);

    console.print_renderable(&table3);

    console.rule("[bold cyan]End Table Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tables_demo_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();
        eprintln!("CAPTURED:\n{}", output);

        assert!(output.contains("Table Features Demo"));
        assert!(output.contains("Star Wars"));
        assert!(output.contains("Movies"));
        assert!(output.contains("George Lucas"));
        assert!(output.contains("Server Status"));
        assert!(output.contains("Online"));
        assert!(output.contains("Rounded"));
        assert!(output.contains("System started"));
        assert!(output.contains("End Table Demo"));
    }
}
