use rich_rust::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let console = Console::new().force_color(true);

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
        progress.print();

        // Wait
        thread::sleep(Duration::from_millis(100));
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
