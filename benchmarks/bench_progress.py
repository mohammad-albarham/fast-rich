import time
import rich_rust
from rich.progress import Progress as RichProgress
import threading

TASKS = 100
STEPS = 100

def bench_python_progress():
    # Use context manager for auto-cleanup
    with RichProgress(transient=True) as progress:
        task = progress.add_task("Processing", total=STEPS)
        for _ in range(STEPS):
            progress.update(task, advance=1)
            # time.sleep(0.001)

def bench_rust_progress():
    progress = rich_rust.Progress()
    # Note: rich_rust bindings currently require manual management or separate thread for repaint?
    # The pure rust implementation handles repainting in background thread if configured?
    # rich_rust::Progress default might not start a thread automatically unless `start()` is called?
    # Let's verify rust implementation.
    
    # Simple benchmark of valid API calls
    task = progress.add_task("Processing", STEPS)
    for i in range(STEPS):
        progress.update(task, i + 1)
        # progress.print() # Manual print for now

if __name__ == "__main__":
    start = time.time()
    for _ in range(TASKS):
        bench_python_progress()
    print(f"Python Progress: {time.time() - start:.4f}s")
    
    start = time.time()
    for _ in range(TASKS):
        bench_rust_progress()
    print(f"Rust Progress:   {time.time() - start:.4f}s")
