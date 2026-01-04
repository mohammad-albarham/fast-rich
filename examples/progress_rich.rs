use fast_rich::console::Console;
use fast_rich::progress::{
    BarColumn, DownloadColumn, FileSizeColumn, MofNColumn, PercentageColumn, Progress,
    SpinnerColumn, TextColumn, TimeRemainingColumn, TotalFileSizeColumn, TransferSpeedColumn,
};
use std::thread;
use std::time::Duration;

fn main() {
    let console = Console::new();
    console.println("[bold cyan]Rich Progress Enhancement Demo (P2 + P3)[/]");
    console.println("[dim]==========================================\n[/]");

    // Demo 1: Standard progress with refresh_per_second setting
    console.println("[yellow]Demo 1: Standard Progress Bar[/]");
    console.println("");
    
    let mut progress = Progress::new()
        .with_console(Console::new())
        .refresh_per_second(10.0)  // P2: Configure refresh rate
        .with_columns(vec![
            Box::new(SpinnerColumn::new()),
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(40)),
            Box::new(PercentageColumn::new()),
            Box::new(MofNColumn::new()),
            Box::new(TransferSpeedColumn),
            Box::new(TimeRemainingColumn),
        ]);

    let task1 = progress.add_task("Downloading file1.zip", Some(100));
    let task2 = progress.add_task("Downloading file2.zip", Some(200));
    let task3 = progress.add_task("Processing data...", None); // Indeterminate

    progress.start();

    let mut completed1 = 0;
    let mut completed2 = 0;

    for _ in 0..100 {
        if completed1 < 100 {
            completed1 += 1;
            progress.update(task1, completed1);
        }
        if completed2 < 200 {
            completed2 += 3;
            if completed2 > 200 {
                completed2 = 200;
            }
            progress.update(task2, completed2);
        }
        progress.refresh();
        thread::sleep(Duration::from_millis(30));
    }

    progress.finish(task1);
    progress.finish(task2);
    progress.finish(task3);
    progress.stop();

    console.println("");
    console.println("[bold green]✓ Demo 1 Complete![/]\n");

    // Demo 2: Sub-character progress with 8th-block Unicode (P2 feature)
    console.println("[yellow]Demo 2: Sub-Character Progress (8th-block Unicode)[/]");
    console.println("");
    
    let mut sub_progress = Progress::new()
        .with_console(Console::new())
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(50).use_sub_blocks(true)),  // P2: Sub-block mode
            Box::new(PercentageColumn::new()),
        ]);

    let smooth_task = sub_progress.add_task("Smooth Progress", Some(1000));

    sub_progress.start();
    for i in 0..=1000 {
        sub_progress.update(smooth_task, i);
        sub_progress.refresh();
        thread::sleep(Duration::from_millis(5));
    }
    sub_progress.stop();

    console.println("");
    console.println("[bold green]✓ Demo 2 Complete![/]\n");

    // Demo 3: Context-manager-style run() method (P2 feature)
    console.println("[yellow]Demo 3: Context Manager run() Method[/]");
    console.println("");
    
    let mut managed_progress = Progress::new()
        .with_console(Console::new())
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(40)),
            Box::new(PercentageColumn::new()),
        ]);

    // P2: Use run() for automatic start/stop lifecycle
    managed_progress.run(|p| {
        let task = p.add_task("Auto-managed task", Some(50));
        for i in 1..=50 {
            p.update(task, i);
            p.refresh(); // Must call refresh manually when auto_refresh thread not implemented
            thread::sleep(Duration::from_millis(50));
        }
        p.finish(task);
    });

    console.println("");
    console.println("[bold green]✓ Demo 3 Complete![/]\n");

    // Demo 4: File Size Columns (P3 feature)
    console.println("[yellow]Demo 4: File Size Columns (P3)[/]");
    console.println("");

    console.println("[dim]Using FileSizeColumn:[/]");
    let mut size_progress = Progress::new()
        .with_console(Console::new())
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(30)),
            Box::new(FileSizeColumn::new()),
        ]);

    let size_task = size_progress.add_task("Reading", Some(10 * 1024 * 1024)); // 10 MB
    size_progress.start();
    for i in 0..50 {
        let bytes = (i + 1) * 200 * 1024; // 200 KB increments
        size_progress.update(size_task, bytes);
        size_progress.refresh();
        thread::sleep(Duration::from_millis(40));
    }
    size_progress.stop();
    console.println("");

    console.println("[dim]Using TotalFileSizeColumn:[/]");
    let mut total_progress = Progress::new()
        .with_console(Console::new())
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(30)),
            Box::new(TotalFileSizeColumn::new()),
        ]);

    let total_task = total_progress.add_task("Copying", Some(50 * 1024 * 1024)); // 50 MB
    total_progress.start();
    for i in 0..50 {
        let bytes = (i + 1) * 1024 * 1024; // 1 MB increments
        total_progress.update(total_task, bytes);
        total_progress.refresh();
        thread::sleep(Duration::from_millis(40));
    }
    total_progress.stop();
    console.println("");

    console.println("[dim]Using DownloadColumn (size @ speed):[/]");
    let mut download_progress = Progress::new()
        .with_console(Console::new())
        .with_columns(vec![
            Box::new(TextColumn::new("[progress.description]")),
            Box::new(BarColumn::new(30)),
            Box::new(DownloadColumn::new()),
        ]);

    let dl_task = download_progress.add_task("Downloading", Some(100 * 1024 * 1024)); // 100 MB
    download_progress.start();
    for i in 0..50 {
        let bytes = (i + 1) * 2 * 1024 * 1024; // 2 MB increments
        download_progress.update(dl_task, bytes);
        download_progress.refresh();
        thread::sleep(Duration::from_millis(50));
    }
    download_progress.stop();

    console.println("");
    console.println("[bold green]✓ Demo 4 Complete![/]\n");
}
