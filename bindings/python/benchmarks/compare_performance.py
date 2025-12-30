"""Benchmark fast_rich vs rich performance."""

from __future__ import annotations

import gc
import statistics
import time
from typing import Callable


def benchmark(func: Callable, iterations: int = 10, warmup: int = 2) -> dict:
    """Run a benchmark and return timing statistics.
    
    Args:
        func: Function to benchmark.
        iterations: Number of iterations.
        warmup: Warmup iterations (not counted).
        
    Returns:
        Dictionary with min, max, mean, median times in ms.
    """
    # Warmup
    for _ in range(warmup):
        func()
        gc.collect()
    
    # Benchmark
    times = []
    for _ in range(iterations):
        gc.collect()
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append((end - start) * 1000)  # Convert to ms
    
    return {
        "min": min(times),
        "max": max(times),
        "mean": statistics.mean(times),
        "median": statistics.median(times),
        "stdev": statistics.stdev(times) if len(times) > 1 else 0,
    }


def format_result(name: str, rich_stats: dict, fast_stats: dict) -> str:
    """Format benchmark result as a row."""
    speedup = rich_stats["median"] / fast_stats["median"] if fast_stats["median"] > 0 else float("inf")
    return (
        f"| {name:<30} | {rich_stats['median']:>8.2f}ms | {fast_stats['median']:>8.2f}ms | "
        f"{speedup:>6.1f}x |"
    )


def run_all_benchmarks():
    """Run all benchmarks and print results."""
    import io
    import sys
    
    print("=" * 70)
    print("fast_rich vs rich Performance Comparison")
    print("=" * 70)
    print()
    
    results = []
    
    # ========== Table Benchmarks ==========
    print("Running Table benchmarks...")
    
    # Small table (10 rows)
    def rich_small_table():
        from rich.console import Console
        from rich.table import Table
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        table = Table(title="Test")
        table.add_column("Name")
        table.add_column("Value")
        for i in range(10):
            table.add_row(f"Item {i}", str(i * 100))
        console.print(table)
    
    def fast_small_table():
        from fast_rich.console import Console
        from fast_rich.table import Table
        out = io.StringIO()
        console = Console(file=out)
        table = Table(title="Test")
        table.add_column("Name")
        table.add_column("Value")
        for i in range(10):
            table.add_row(f"Item {i}", str(i * 100))
        console.print(table)
    
    rich_stats = benchmark(rich_small_table)
    fast_stats = benchmark(fast_small_table)
    results.append(("Table (10 rows)", rich_stats, fast_stats))
    
    # Large table (1000 rows)
    def rich_large_table():
        from rich.console import Console
        from rich.table import Table
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        table = Table()
        table.add_column("ID")
        table.add_column("Name")
        table.add_column("Value")
        for i in range(1000):
            table.add_row(str(i), f"Item {i}", str(i * 100))
        console.print(table)
    
    def fast_large_table():
        from fast_rich.console import Console
        from fast_rich.table import Table
        out = io.StringIO()
        console = Console(file=out)
        table = Table()
        table.add_column("ID")
        table.add_column("Name")
        table.add_column("Value")
        for i in range(1000):
            table.add_row(str(i), f"Item {i}", str(i * 100))
        console.print(table)
    
    rich_stats = benchmark(rich_large_table, iterations=5)
    fast_stats = benchmark(fast_large_table, iterations=5)
    results.append(("Table (1000 rows)", rich_stats, fast_stats))
    
    # ========== Text Benchmarks ==========
    print("Running Text benchmarks...")
    
    def rich_styled_text():
        from rich.console import Console
        from rich.text import Text
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        text = Text()
        for i in range(100):
            text.append(f"Line {i}: ", style="bold")
            text.append("Some styled content\n", style="italic cyan")
        console.print(text)
    
    def fast_styled_text():
        from fast_rich.console import Console
        from fast_rich.text import Text
        out = io.StringIO()
        console = Console(file=out)
        text = Text()
        for i in range(100):
            text.append(f"Line {i}: ", style="bold")
            text.append("Some styled content\n", style="italic cyan")
        console.print(text)
    
    rich_stats = benchmark(rich_styled_text)
    fast_stats = benchmark(fast_styled_text)
    results.append(("Styled Text (100 lines)", rich_stats, fast_stats))
    
    # ========== Panel Benchmarks ==========
    print("Running Panel benchmarks...")
    
    def rich_panel():
        from rich.console import Console
        from rich.panel import Panel
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for i in range(50):
            panel = Panel(f"Content {i}", title=f"Panel {i}")
            console.print(panel)
    
    def fast_panel():
        from fast_rich.console import Console
        from fast_rich.panel import Panel
        out = io.StringIO()
        console = Console(file=out)
        for i in range(50):
            panel = Panel(f"Content {i}", title=f"Panel {i}")
            console.print(panel)
    
    rich_stats = benchmark(rich_panel)
    fast_stats = benchmark(fast_panel)
    results.append(("Panel (50 panels)", rich_stats, fast_stats))
    
    # ========== Tree Benchmarks ==========
    print("Running Tree benchmarks...")
    
    def rich_tree():
        from rich.console import Console
        from rich.tree import Tree
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        tree = Tree("Root")
        for i in range(10):
            branch = tree.add(f"Branch {i}")
            for j in range(10):
                branch.add(f"Leaf {j}")
        console.print(tree)
    
    def fast_tree():
        from fast_rich.console import Console
        from fast_rich.tree import Tree
        out = io.StringIO()
        console = Console(file=out)
        tree = Tree("Root")
        for i in range(10):
            branch = tree.add(f"Branch {i}")
            for j in range(10):
                branch.add(f"Leaf {j}")
        console.print(tree)
    
    rich_stats = benchmark(rich_tree)
    fast_stats = benchmark(fast_tree)
    results.append(("Tree (10x10 nodes)", rich_stats, fast_stats))
    
    # ========== Print Results ==========
    print()
    print("| Benchmark                      |     rich   | fast_rich  | Speedup |")
    print("| :----------------------------- | ---------: | ---------: | ------: |")
    for name, rich_s, fast_s in results:
        print(format_result(name, rich_s, fast_s))
    
    print()
    print("Note: Times are median values from multiple iterations.")
    print("      Speedup = rich_time / fast_rich_time")


if __name__ == "__main__":
    run_all_benchmarks()
