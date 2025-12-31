use rich_rust::prelude::*;
use rich_rust::layout::Layout;

fn main() {
    let console = Console::new().force_color(true);
    
    console.rule("[bold green]Panels & Layout Demo[/]");
    console.newline();

    // 1. Panel Variations
    console.print("[bold]1. Panel Variations[/]");
    
    let p1 = Panel::new("Simple Panel").title("Title");
    
    let p2 = Panel::new(rich_rust::markup::parse("Panel with [blue]styled[/] content and [bold]subtitle[/]"))
        .title("Styled")
        .subtitle("Footer")
        .border_style(BorderStyle::Double)
        .padding(2, 1);
        
    let p3 = Panel::new("Fit-to-content Panel")
        .title("Compact")
        .expand(false)
        .border_style(BorderStyle::Rounded)
        .style(Style::new().foreground(Color::Cyan));
        
    console.print_renderable(&p1);
    console.print_renderable(&p2);
    console.print_renderable(&p3);
    console.newline();

    // 2. Layout (Split Views)
    console.print("[bold]2. Layout (Splits)[/]");
    
    // Create lead layout
    let mut root = Layout::new();
    root.split_row(vec![
        Layout::new().with_name("Left").with_name("Left"),
        Layout::new().with_name("Right").with_name("Right"),
    ]);

    // Update left column
    let mut left = Layout::new();
    left.update(Panel::new("Left Column\nRow 1\nRow 2"));
    
    // Update right column
    let mut right = Layout::new();
    right.update(Panel::new("Right Column\nOnly 1 Row"));

    // In a real layout engine we'd attach these, but current Layout implementation 
    // is a tree that renders children.
    // The current Rust implementation of Split is basic.
    // Let's manually render them side-by-side using Columns if available, otherwise stack
    console.print("[dim]Note: Layout engine is WIP, stacking panels:[/]");
    
    console.print_renderable(&Panel::new("Top Section"));
    
    // Columns (if supported)
    // The current codebase has 'columns' module
    #[cfg(feature = "std")]
    {
       // If Columns implemented
    }

    console.print_renderable(&left);
    console.print_renderable(&right);
    
    console.rule("[bold green]End Panel Demo[/]");
}
