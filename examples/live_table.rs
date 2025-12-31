use fast_rich::prelude::*;
use std::thread;
use std::time::Duration;

pub fn run(console: &Console) {
    console.clear();
    console.print("[bold yellow]Live Table Demo (Interactive Animation)[/]");

    // We can't really "test" animation in CI easily without mocking time/cursor.
    // So we just run one frame for tests, and full loop for real run.
    let is_test =
        std::env::var("CARGO_MANIFEST_DIR").is_ok() && std::env::args().any(|a| a == "--test");
    // Actually, cargo test runs this function.

    let mut live = fast_rich::live::Live::new(Table::new(), console);
    live.start();

    let mut table = Table::new();
    table.add_column("ID");
    table.add_column("Status");
    table.add_column("Progress");

    // Initial state
    table.add_row_strs(&["1", "Downloading", "0%"]);
    table.add_row_strs(&["2", "Pending", "0%"]);
    table.add_row_strs(&["3", "Pending", "0%"]);

    live.update(table.clone());
    live.refresh();

    if !is_test {
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(200));

            // Update table content
            let mut t = Table::new();
            t.add_column("ID");
            t.add_column("Status");
            t.add_column("Progress");

            let p1 = std::cmp::min(100, i * 10);
            let p2 = std::cmp::min(100, i * 5);

            t.add_row_strs(&[
                "1",
                if p1 < 100 { "Downloading" } else { "Done" },
                &format!("{}%", p1),
            ]);
            t.add_row_strs(&[
                "2",
                if p2 < 100 { "Downloading" } else { "Done" },
                &format!("{}%", p2),
            ]);

            if i > 5 {
                t.add_row_strs(&["3", "Starting...", "10%"]);
            } else {
                t.add_row_strs(&["3", "Pending", "0%"]);
            }

            live.update(t);
            live.refresh();
        }
    }

    live.stop();
    console.print("[bold green]Live demo finished![/]");
}

fn main() {
    let console = Console::new();
    run(&console);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_table_output() {
        // We can't easily capture output of Live because it writes directly to stdout (crossterm).
        // But we can verify it doesn't panic.
        let console = Console::new();
        // Just run it. It captures stdout in test harness anyway mostly?
        // Crossterm writes to /dev/tty often on unix.
        // We accept that this test might just ensure no panic.
        run(&console);
    }
}
