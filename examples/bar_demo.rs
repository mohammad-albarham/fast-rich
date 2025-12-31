use rich_rust::bar::{BarChart, BarData};
use rich_rust::prelude::*;

fn main() {
    let console = Console::new().width(70);
    console.rule("[bold blue]Bar Chart Demo[/]");

    // Example 1: Simple bars
    console.print("[bold]Monthly Sales:[/]");
    let mut chart1 = BarChart::new();
    chart1.bar("Jan", 45.5);
    chart1.bar("Feb", 62.3);
    chart1.bar("Mar", 78.9);
    chart1.bar("Apr", 55.2);
    chart1.bar("May", 91.7);
    console.print_renderable(&chart1);
    console.newline();

    // Example 2: Custom colors
    console.print("[bold]System Resources:[/]");
    let mut chart2 = BarChart::new().width(40);
    chart2.add_bar(BarData::new("CPU", 75.5).color(Color::Red));
    chart2.add_bar(BarData::new("Memory", 45.2).color(Color::Yellow));
    chart2.add_bar(BarData::new("Disk", 92.8).color(Color::Magenta));
    chart2.add_bar(BarData::new("Network", 30.1).color(Color::Cyan));
    console.print_renderable(&chart2);
    console.newline();

    // Example 3: Different bar character
    console.print("[bold]Progress Bars:[/]");
    let mut chart3 = BarChart::new()
        .bar_char('▓')
        .default_color(Color::BrightGreen)
        .show_values(false);
    chart3.bar("Task 1", 100.0);
    chart3.bar("Task 2", 75.0);
    chart3.bar("Task 3", 50.0);
    chart3.bar("Task 4", 25.0);
    console.print_renderable(&chart3);
    console.newline();

    // Example 4: Large dataset
    console.print("[bold]Weekly Activity:[/]");
    let mut chart4 = BarChart::new().default_color(Color::BrightBlue);
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let values = [120.0, 150.0, 180.0, 165.0, 195.0, 85.0, 45.0];
    for (day, &value) in days.iter().zip(values.iter()) {
        chart4.bar(*day, value);
    }
    console.print_renderable(&chart4);

    console.rule("[bold blue]End Demo[/]");
}
