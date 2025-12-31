use rich_rust::padding::{Padding, PaddingSpec};
use rich_rust::prelude::*;

fn main() {
    let console = Console::new().width(60);
    console.rule("[bold red]Padding Demo[/]");

    // 1. All sides equal
    console.print("[bold]Padding::all(2):[/]");
    let text1 =
        Text::plain("Padded on all sides").style(Style::new().bold().foreground(Color::Green));
    let padded1 = Padding::all(text1, 2);
    console.print_renderable(&padded1);
    console.newline();

    // 2. Symmetric (vertical, horizontal)
    console.print("[bold]Padding::symmetric(1, 4):[/]");
    let text2 =
        Text::plain("Vertical: 1, Horizontal: 4").style(Style::new().foreground(Color::Cyan));
    let padded2 = Padding::symmetric(text2, 1, 4);
    console.print_renderable(&padded2);
    console.newline();

    // 3. Individual sides
    console.print("[bold]PaddingSpec::new(2, 8, 1, 3):[/]");
    let text3 =
        Text::plain("Custom padding per side").style(Style::new().foreground(Color::Magenta));
    let padded3 = Padding::new(text3, PaddingSpec::new(2, 8, 1, 3));
    console.print_renderable(&padded3);
    console.newline();

    // 4. Padding around a panel
    console.print("[bold]Padding around Panel:[/]");
    let panel_content =
        Text::plain("Panel with padding").style(Style::new().foreground(Color::Yellow));
    let panel = Panel::new(panel_content).title("Info");
    let padded_panel = Padding::all(panel, 1);
    console.print_renderable(&padded_panel);

    console.rule("[bold red]End Demo[/]");
}
