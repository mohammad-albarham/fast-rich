use fast_rich::prelude::*;

fn main() -> std::io::Result<()> {
    let console = Console::new().record(true);

    console.rule("[bold red]Export Demo[/]");
    console.print("This output is being [bold green]recorded[/]!");

    let mut table = Table::new();
    table.add_column("Language");
    table.add_column("Features");
    table.add_row_strs(&["Rust", "Fast & Safe"]);
    table.add_row_strs(&["Python", "Easy & Rich"]);

    console.print_renderable(&table);

    console.print("Check [bold cyan]export.html[/] and [bold magenta]export.svg[/] files!");

    console.save_html("export.html")?;
    console.save_svg("export.svg")?;

    println!("Exported to export.html and export.svg");
    Ok(())
}
