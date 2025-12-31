use fast_rich::prelude::*;

fn run(console: &Console) {
    console.rule("[bold cyan]Logging & Inspect Demo[/]");
    console.newline();

    // 1. Inspection
    console.print("[bold yellow]1. Inspect/Debug[/]");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Config {
        host: String,
        port: u16,
        active: bool,
    }

    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        active: true,
    };

    console.print("Inspecting struct:");
    console.print(&format!("[dim]{:?}[/]", config));
    console.newline();

    // 2. Logging Levels
    console.print("[bold yellow]2. Logging Levels[/]");

    // Simulate log messages
    fn log(console: &Console, level: &str, msg: &str) {
        let style = match level {
            "DEBUG" => "dim blue",
            "INFO" => "green",
            "WARN" => "bold yellow",
            "ERROR" => "bold red",
            _ => "white",
        };

        let ts = "12:00:00";
        // Fixed format string
        console.print(&format!("[dim]{}[/] [{}]{}[/] {}", ts, style, level, msg));
    }

    log(console, "DEBUG", "Connecting to server...");
    log(console, "INFO", "Connection established.");
    log(console, "WARN", "Latency high (150ms).");
    log(console, "ERROR", "Connection dropped!");

    console.rule("[bold cyan]End Logging Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_demo_output() {
        let console = Console::capture();
        run(&console);
        let output = console.get_captured_output();

        // Debug output if needed
        // eprintln!("{}", output);

        assert!(output.contains("Logging & Inspect Demo"));
        assert!(output.contains("Inspect/Debug"));
        assert!(output.contains("Config"));
        assert!(output.contains("localhost"));
        assert!(output.contains("Logging"));
        assert!(output.contains("Levels"));
        assert!(output.contains("INFO"));
        assert!(output.contains("Connection established"));
        assert!(output.contains("End Logging Demo"));
    }
}
