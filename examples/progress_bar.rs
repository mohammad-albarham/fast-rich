use fast_rich::prelude::*;
use std::thread;
use std::time::Duration;

fn run(console: &Console, delay: Duration) {
    console.rule("[bold green]Progress Bar Demo[/]");
    console.newline();

    console.print("Simulating a multi-step download process...");
    console.newline();

    let progress = Progress::new();

    // Add tasks
    let download_id = progress.add_task("[cyan]Downloading[/]", Some(100));
    let extract_id = progress.add_task("[green]Extracting[/]", Some(50));
    let install_id = progress.add_task("[yellow]Installing[/]", Some(20));

    // Simulation Loop
    // In a real app you'd run this in a loop or threads.
    // Here we manually advance them to show frames.

    let mut d_completed = 0;
    let mut e_completed = 0;

    let total_steps = 20;

    for _ in 0..total_steps {
        // Update tasks
        progress.advance(download_id, 5); // +5%
        d_completed += 5;

        if d_completed > 50 {
            progress.advance(extract_id, 2);
            e_completed += 2;
        }

        if e_completed > 25 {
            progress.advance(install_id, 1);
        }

        // Render current frame
        // In simulation, we print manual updates
        // Note: Progress::print() normally overwrites or prints.
        // For testing/example simplicity we just print.
        progress.print();

        // Wait
        thread::sleep(delay);
    }

    progress.finish(download_id);
    progress.finish(extract_id);
    progress.finish(install_id);

    // Final render
    progress.print();
    console.newline();
    console.print("[bold green]Done![/]");

    console.rule("[bold green]End Progress Demo[/]");
}

fn main() {
    let console = Console::new().force_color(true);
    run(&console, Duration::from_millis(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_output() {
        let console = Console::capture();
        // Run with minimal delay for speed
        run(&console, Duration::from_micros(1));
        let output = console.get_captured_output();

        assert!(output.contains("Progress Bar Demo"));
        assert!(output.contains("Simulating"));
        assert!(output.contains("Done!"));
        assert!(output.contains("End Progress Demo"));
    }
}
