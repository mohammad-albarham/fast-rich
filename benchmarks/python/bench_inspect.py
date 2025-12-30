from rich_rust import inspect, Console
import time
from rich import inspect as rich_inspect
from rich.console import Console as RichConsole

console = Console()
rich_console = RichConsole(file=open("/dev/null", "w"))

class LargeObject:
    def __init__(self):
        self.data = list(range(100))
        self.attr = "test" * 50

obj = LargeObject()

def bench_inspect(name, iterations=1000):
    start = time.time()
    for _ in range(iterations):
        # We capture stdout to avoid spam, or assume inspect prints to console
        # Rust bind inspect creates new console inside currently (inefficient for bench but ok for compare)
        inspect(obj)
    rust_time = time.time() - start

    start = time.time()
    for _ in range(iterations):
        rich_inspect(obj, console=rich_console)
    py_time = time.time() - start

    print(f"{name}: Rust={rust_time:.4f}s Python={py_time:.4f}s Speedup={py_time/rust_time:.2f}x")

if __name__ == "__main__":
    bench_inspect("Inspect Large Object")
