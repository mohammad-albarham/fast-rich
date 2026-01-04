//! Minimal track() demo for documentation GIF

use fast_rich::progress::track;
use std::thread;
use std::time::Duration;

fn main() {
    // Python-style progress: for item in track(range(30), description="...")
    for _item in track(0..30, "Processing items") {
        thread::sleep(Duration::from_millis(80));
    }
}
