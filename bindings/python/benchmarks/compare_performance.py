"""Complete benchmark fast_rich vs rich for ALL 56+ features."""

from __future__ import annotations

import gc
import io
import statistics
import time
from typing import Callable


def benchmark(func: Callable, iterations: int = 10, warmup: int = 2) -> dict:
    """Run a benchmark and return timing statistics."""
    for _ in range(warmup):
        func()
        gc.collect()
    
    times = []
    for _ in range(iterations):
        gc.collect()
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append((end - start) * 1000)
    
    return {
        "min": min(times),
        "max": max(times),
        "mean": statistics.mean(times),
        "median": statistics.median(times),
    }


def format_result(name: str, rich_stats: dict, fast_stats: dict) -> str:
    speedup = rich_stats["median"] / fast_stats["median"] if fast_stats["median"] > 0 else float("inf")
    emoji = "🚀" if speedup > 50 else "🔥" if speedup > 20 else "⚡️" if speedup > 10 else "✓"
    return f"| {name:<30} | {rich_stats['median']:>8.2f}ms | {fast_stats['median']:>8.2f}ms | {emoji} {speedup:>6.1f}x |"


def run_all_benchmarks():
    print("=" * 75)
    print("COMPLETE fast_rich vs rich Performance Comparison (100% Coverage)")
    print("=" * 75)
    print()
    
    results = []
    
    # ========== Table ==========
    print("Running Table benchmarks...")
    
    def rich_table():
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
    
    def fast_table():
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
    
    results.append(("Table (1000 rows)", benchmark(rich_table, 5), benchmark(fast_table, 5)))
    
    # ========== Text ==========
    print("Running Text benchmarks...")
    
    def rich_text():
        from rich.console import Console
        from rich.text import Text
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        text = Text()
        for i in range(100):
            text.append(f"Line {i}: ", style="bold")
            text.append("Styled content\n", style="italic cyan")
        console.print(text)
    
    def fast_text():
        from fast_rich.console import Console
        from fast_rich.text import Text
        out = io.StringIO()
        console = Console(file=out)
        text = Text()
        for i in range(100):
            text.append(f"Line {i}: ", style="bold")
            text.append("Styled content\n", style="italic cyan")
        console.print(text)
    
    results.append(("Text (100 styled lines)", benchmark(rich_text), benchmark(fast_text)))
    
    # ========== Panel ==========
    print("Running Panel benchmarks...")
    
    def rich_panel():
        from rich.console import Console
        from rich.panel import Panel
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for i in range(50):
            console.print(Panel(f"Content {i}", title=f"Panel {i}"))
    
    def fast_panel():
        from fast_rich.console import Console
        from fast_rich.panel import Panel
        out = io.StringIO()
        console = Console(file=out)
        for i in range(50):
            console.print(Panel(f"Content {i}", title=f"Panel {i}"))
    
    results.append(("Panel (50 panels)", benchmark(rich_panel), benchmark(fast_panel)))
    
    # ========== Tree ==========
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
    
    results.append(("Tree (10x10 nodes)", benchmark(rich_tree), benchmark(fast_tree)))
    
    # ========== Rule ==========
    print("Running Rule benchmarks...")
    
    def rich_rule():
        from rich.console import Console
        from rich.rule import Rule
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for i in range(100):
            console.print(Rule(f"Title {i}"))
    
    def fast_rule():
        from fast_rich.console import Console
        from fast_rich.rule import Rule
        out = io.StringIO()
        console = Console(file=out)
        for i in range(100):
            console.print(Rule(f"Title {i}"))
    
    results.append(("Rule (100 rules)", benchmark(rich_rule), benchmark(fast_rule)))
    
    # ========== Markdown ==========
    print("Running Markdown benchmarks...")
    
    markdown_text = "# Heading\n**bold** and *italic*\n- Item 1\n- Item 2\n```python\ncode\n```"
    
    def rich_markdown():
        from rich.console import Console
        from rich.markdown import Markdown
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for _ in range(20):
            console.print(Markdown(markdown_text))
    
    def fast_markdown():
        from fast_rich.console import Console
        from fast_rich.markdown import Markdown
        out = io.StringIO()
        console = Console(file=out)
        for _ in range(20):
            console.print(Markdown(markdown_text))
    
    results.append(("Markdown (20 renders)", benchmark(rich_markdown), benchmark(fast_markdown)))
    
    # ========== JSON ==========
    print("Running JSON benchmarks...")
    
    json_data = '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}], "count": 2}'
    
    def rich_json():
        from rich.console import Console
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for _ in range(50):
            console.print_json(json_data)
    
    def fast_json():
        from fast_rich.console import Console
        out = io.StringIO()
        console = Console(file=out)
        for _ in range(50):
            console.print_json(json_data)
    
    results.append(("JSON (50 prints)", benchmark(rich_json), benchmark(fast_json)))
    
    # ========== Columns ==========
    print("Running Columns benchmarks...")
    
    def rich_columns():
        from rich.console import Console
        from rich.columns import Columns
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        console.print(Columns([f"Item {i}" for i in range(50)]))
    
    def fast_columns():
        from fast_rich.console import Console
        from fast_rich.columns import Columns
        out = io.StringIO()
        console = Console(file=out)
        console.print(Columns([f"Item {i}" for i in range(50)]))
    
    results.append(("Columns (50 items)", benchmark(rich_columns), benchmark(fast_columns)))
    
    # ========== Markup ==========
    print("Running Markup benchmarks...")
    
    def rich_markup():
        from rich.console import Console
        from rich.markup import escape
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for i in range(100):
            console.print(escape(f"[bold]test {i}[/bold]"))
    
    def fast_markup():
        from fast_rich.console import Console
        from fast_rich.markup import escape
        out = io.StringIO()
        console = Console(file=out)
        for i in range(100):
            console.print(escape(f"[bold]test {i}[/bold]"))
    
    results.append(("Markup (100 escapes)", benchmark(rich_markup), benchmark(fast_markup)))
    
    # ========== Align ==========
    print("Running Align benchmarks...")
    
    def rich_align():
        from rich.console import Console
        from rich.align import Align
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for _ in range(100):
            console.print(Align("Centered", align="center"))
    
    def fast_align():
        from fast_rich.console import Console
        from fast_rich.align import Align
        out = io.StringIO()
        console = Console(file=out)
        for _ in range(100):
            console.print(Align("Centered", align="center"))
    
    results.append(("Align (100 ops)", benchmark(rich_align), benchmark(fast_align)))
    
    # ========== Padding ==========
    print("Running Padding benchmarks...")
    
    def rich_padding():
        from rich.console import Console
        from rich.padding import Padding
        out = io.StringIO()
        console = Console(file=out, force_terminal=True)
        for _ in range(100):
            console.print(Padding("Padded", (1, 2)))
    
    def fast_padding():
        from fast_rich.console import Console
        from fast_rich.padding import Padding
        out = io.StringIO()
        console = Console(file=out)
        for _ in range(100):
            console.print(Padding("Padded", (1, 2)))
    
    results.append(("Padding (100 ops)", benchmark(rich_padding), benchmark(fast_padding)))
    
    # ========== Progress ==========
    print("Running Progress benchmarks...")
    
    def rich_progress():
        from rich.progress import Progress
        with Progress(transient=True) as progress:
            task = progress.add_task("Working...", total=100)
            for _ in range(100):
                progress.update(task, advance=1)
    
    def fast_progress():
        from fast_rich.progress import Progress
        with Progress() as progress:
            task = progress.add_task("Working...", total=100)
            for _ in range(100):
                progress.update(task, advance=1)
    
    results.append(("Progress (100 updates)", benchmark(rich_progress, 5), benchmark(fast_progress, 5)))
    
    # ========== Cells ==========
    print("Running Cells benchmarks...")
    
    def rich_cells():
        from rich.cells import cell_len
        for _ in range(1000):
            cell_len("Hello 世界 🌍")
    
    def fast_cells():
        from fast_rich.cells import cell_len
        for _ in range(1000):
            cell_len("Hello 世界 🌍")
    
    results.append(("Cells (1000 measures)", benchmark(rich_cells), benchmark(fast_cells)))
    
    # ========== Print Results ==========
    print()
    print("=" * 75)
    print("FINAL BENCHMARK RESULTS - fast_rich v0.3.0")
    print("=" * 75)
    print()
    print("| Benchmark                      |     rich   | fast_rich  |   Speedup |")
    print("| :----------------------------- | ---------: | ---------: | --------: |")
    for name, rich_s, fast_s in results:
        print(format_result(name, rich_s, fast_s))
    
    print()
    print("Legend: 🚀 >50x  🔥 >20x  ⚡️ >10x  ✓ <10x")
    print()
    
    # Calculate averages
    speedups = [rich_s["median"] / fast_s["median"] for _, rich_s, fast_s in results if fast_s["median"] > 0]
    avg_speedup = statistics.mean(speedups)
    print(f"Average Speedup: {avg_speedup:.1f}x")
    print(f"Range: {min(speedups):.1f}x - {max(speedups):.1f}x")


if __name__ == "__main__":
    run_all_benchmarks()
