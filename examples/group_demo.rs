use rich_rust::prelude::*;
use rich_rust::group::{RenderGroup, Fit};

fn main() {
    let console = Console::new().width(70);
    console.rule("[bold cyan]Render Groups Demo[/]");

    // Example 1: Simple group with spacing
    console.print("[bold]Group with spacing:[/]");
    let mut group1 = RenderGroup::new().spacing(1);
    group1.add(Text::plain("Item 1").style(Style::new().foreground(Color::Green)));
    group1.add(Text::plain("Item 2").style(Style::new().foreground(Color::Yellow)));
    group1.add(Text::plain("Item 3").style(Style::new().foreground(Color::Red)));
    console.print_renderable(&group1);
    console.newline();

    // Example 2: Group with dividers
    console.print("[bold]Group with dividers:[/]");
    let mut group2 = RenderGroup::new().divider("─");
    group2.add(Panel::new(Text::plain("Section 1")).title("First"));
    group2.add(Panel::new(Text::plain("Section 2")).title("Second"));
    group2.add(Panel::new(Text::plain("Section 3")).title("Third"));
    console.print_renderable(&group2);
    console.newline();

    // Example 3: Mixed content with spacing and dividers
    console.print("[bold]Mixed content:[/]");
    let mut group3 = RenderGroup::new()
        .spacing(1)
        .divider("═")
        .fit(Fit::Fill);
    
    let mut table = Table::new();
    table.add_column("Name");
    table.add_column("Value");
    table.add_row_strs(&["Speed", "Fast"]);
    table.add_row_strs(&["Safety", "High"]);
    
    group3.add(Text::plain("Configuration:").style(Style::new().bold()));
    group3.add(table);
    group3.add(Text::plain("Status: Ready").style(Style::new().foreground(Color::Green)));
    
    console.print_renderable(&group3);
    console.newline();

    // Example 4: Nested panels
    console.print("[bold]Nested content:[/]");
    let mut group4 = RenderGroup::new().spacing(2);
    
    for i in 1..=3 {
        let panel = Panel::new(
            Text::plain(format!("Content for panel {}", i))
                .style(Style::new().foreground(Color::Cyan))
        ).title(&format!("Panel {}", i));
        group4.add(panel);
    }
    
    console.print_renderable(&group4);

    console.rule("[bold cyan]End Demo[/]");
}
