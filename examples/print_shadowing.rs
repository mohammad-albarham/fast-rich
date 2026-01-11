//! Demonstrates using fast-rich's print!/println! macros as drop-in replacements
//! for the standard library versions.
//!
//! Run with: cargo run --example print_shadowing

// Shadow the standard library print macros with rich versions
use fast_rich::{print, println};

fn main() {
    // Basic rich markup
    println!("[bold green]Success![/] All systems operational.");
    println!();

    // Format arguments work exactly like std::println!
    let name = "Alice";
    let score = 100;
    println!(
        "[cyan]Player[/]: {} scored [yellow]{}[/] points",
        name, score
    );
    println!();

    // Named arguments
    println!(
        "[magenta]{role}[/] {name} completed [blue]{task}[/]",
        role = "Admin",
        name = "Bob",
        task = "backup"
    );
    println!();

    // Colors and styles
    println!("[red]Error[/] | [yellow]Warning[/] | [green]Info[/] | [blue]Debug[/]");
    println!();

    // print! without newline for inline progress
    print!("[blue]Processing[/]");
    for _ in 0..3 {
        print!(".");
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
    println!(" [bold green]Done![/]");
    println!();

    // Complex formatting
    println!(
        "[bold magenta]Statistics[/]:\n  • Items: [cyan]{}[/]\n  • Status: [green]{}[/]",
        42, "Active"
    );
    println!();

    // ==========================================================================
    // Handling data with brackets (like debug output)
    // ==========================================================================
    //
    // The markup parser is smart: it ignores brackets if they don't look like styles.
    // So simple debug output often "just works":

    let data = vec![1, 2, 3, 4, 5];
    println!("[yellow]Vector (auto):[/] {:?}", data); // Works thanks to smart parser!

    // However, for strict raw output (or if data might contain valid style syntax),
    // use `println_raw!` to skip markup parsing entirely:
    use fast_rich::println_raw;

    let map = std::collections::HashMap::from([("a", 1), ("b", 2)]);

    print!("[cyan]HashMap (raw):[/] ");
    println_raw!("{:?}", map);

    // ✅ Regular values without brackets work fine with styled println!
    println!("[green]Length:[/] {}", data.len());
    println!();

    // ==========================================================================
    // LIMITATION: Unintended Tag Collisions
    // ==========================================================================
    // If your debug data looks exactly like a style tag, the "smart" parser
    // will be fooled and consume it.
    //
    // Example: A singleton Vec of an Enum named "Red".
    // Debug output is "[Red]". "Red" is a valid color.

    #[derive(Debug)]
    enum Status {
        Red,
    }
    let status = vec![Status::Red];

    // ⚠️ PROBLEM: {:?} prints "[Red]" -> interpreted as color red!
    // The parser consumes [Red], text becomes red, and content disappears.
    print!("Problem: ");
    println!("{:?}", status);

    // ✅ SOLUTION: Use `println_raw!` to force literal output
    print!("Fixed:   ");
    println_raw!("{:?}", status);
    println!();

    // ==========================================================================
    // Printing to Stderr
    // ==========================================================================
    use fast_rich::{eprint, eprintln};

    eprintln!("[bold red]Critical Error:[/] Connection failed");
    eprint!("[bold magenta]Loading stderr...[/]");
    eprintln!("[green]OK[/]");

    // Using raw stderr macro for data
    use fast_rich::eprintln_raw;
    eprintln_raw!("Error Context: {:?}", vec!["timeout", "retry"]);
    println!();

    // ==========================================================================
    // Using aliases when you need both std and rich
    // ==========================================================================
    use fast_rich::rprintln;
    std::println!("Standard print (no markup): [bold]this is literal[/]");
    rprintln!("Rich print: [bold]this is bold[/]");
}
