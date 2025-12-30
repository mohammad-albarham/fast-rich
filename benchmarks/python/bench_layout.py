from rich_rust import Layout, Console
import time
from rich.layout import Layout as RichLayout
from rich.console import Console as RichConsole

console = Console()
rich_console = RichConsole(file=open("/dev/null", "w"))

def bench_layout(name, iterations=1000):
    start = time.time()
    for _ in range(iterations):
        layout = Layout()
        # Simulate a split
        # layout.split_column(...) # Bindings for split not full yet, so just creation/render
        console.print(layout)
    rust_time = time.time() - start

    start = time.time()
    for _ in range(iterations):
        layout = RichLayout()
        rich_console.print(layout)
    py_time = time.time() - start

    print(f"{name}: Rust={rust_time:.4f}s Python={py_time:.4f}s Speedup={py_time/rust_time:.2f}x")

if __name__ == "__main__":
    bench_layout("Layout Render")
