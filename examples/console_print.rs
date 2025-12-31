use rich_rust::console::Console;

fn run(console: &Console) {
    console.rule("[bold magenta]Console API Demo[/]");
    console.print("This is a [bold]regular print[/].");
    console.newline();

    console.println("This is a [italic]println[/] which adds a newline.");

    console.rule("Padding");
    console.print_renderable(&rich_rust::text::Text::from("Explicit Renderable"));
    console.newline();

    // Demonstrate some emoji if enabled
    console.println("Emoji: :snake: :crab:");
}

fn main() {
    let console = Console::new();
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_print_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();
        eprintln!("Captured output: {:?}", output);

        // Check Rule
        assert!(output.contains("Console API Demo"));
        // ...

        // Check Print/Println
        assert!(output.contains("regular"));
        assert!(output.contains("print"));
        assert!(output.contains("println"));

        // Remove debug print

        // Check Renderable
        assert!(output.contains("Explicit Renderable"));
    }
}
