use fast_rich::nested_progress::NestedProgress;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Nested Progress Demo ===\n");

    // Create root progress
    let mut root = NestedProgress::new("Main Task", 100);

    println!("Starting nested progress demonstration...\n");

    // Simulate work with nested tasks
    for i in 0..3 {
        let _child = root.add_child(format!("Subtask {}", i + 1), 50);

        for j in 0..5 {
            thread::sleep(Duration::from_millis(100));
            println!("  Subtask {} progress: {}/50", i + 1, (j + 1) * 10);
        }

        root.update(33); // Update parent
    }

    println!("\nProgress: {:.1}%", root.percent());
    println!(
        "Nested progress complete! ({} subtasks)",
        root.child_count()
    );
}
