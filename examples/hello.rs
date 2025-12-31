use fast_rich::console::Console;

// Logic extracted for testability
fn run(console: &Console) {
    console.print("[bold green]Hello[/], [blue]World[/]!");
    console.newline();
}

fn main() {
    let console = Console::new();
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
        // Basic check for color codes or captured structure if needed
    }
}
