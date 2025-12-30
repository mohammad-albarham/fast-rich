import time
import rich_rust
from rich.console import Console as RichConsole

ITERATIONS = 10000

def bench_python_logging():
    console = RichConsole(file=open('/dev/null', 'w'))
    for i in range(ITERATIONS):
        console.log(f"Log message {i}")

def bench_rust_logging():
    console = rich_rust.Console()
    # Assuming rust bindings direct log to stdout?
    # We might want to suppress output for benchmarks to measure overhead only.
    # But `Console` currently defaults to stdout.
    
    for i in range(ITERATIONS):
        console.log(f"Log message {i}")

if __name__ == "__main__":
    start = time.time()
    bench_python_logging()
    print(f"Python Logging: {time.time() - start:.4f}s")
    
    # Run Rust benchmark
    # Note: Rust console writes to stdout, redirecting in shell if needed.
    start = time.time()
    bench_rust_logging()
    print(f"Rust Logging:   {time.time() - start:.4f}s")
