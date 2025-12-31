use rich_rust::console::Console;
use rich_rust::progress::{
    Progress, SpinnerColumn, BarColumn, TextColumn, 
    PercentageColumn, TimeRemainingColumn, TransferSpeedColumn, MofNColumn
};
use std::thread;
use std::time::Duration;

fn main() {
    let mut console = Console::new();
    println!("Rich Progress Enhancement Demo\n============================");

    let progress = Progress::new().with_columns(vec![
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

    let mut completed1 = 0;
    let mut completed2 = 0;
    
    // Simulation loop
    let steps = 100;
    for _ in 0..steps {
        if completed1 < 100 {
            completed1 += 1;
            progress.update(task1, completed1);
        }
        
        if completed2 < 200 {
            completed2 += 3;
            if completed2 > 200 { completed2 = 200; }
            progress.update(task2, completed2);
        }
        
        // Spinners update automatically based on elapsed time if we call print()
        progress.print();
        thread::sleep(Duration::from_millis(50));
    }
    
    progress.finish(task1);
    progress.finish(task2);
    progress.finish(task3);
    
    progress.print(); // Final print
    
    println!("\nDone");
}
