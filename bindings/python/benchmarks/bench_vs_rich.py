"""Compare print times of fast_rich and rich.

Usage: python benchmarks/bench_vs_rich.py rich
       python benchmarks/bench_vs_rich.py fast_rich
(build fast_rich with `maturin develop --release` first)
"""
import io, sys, time, importlib
pkg = sys.argv[1]
Console = importlib.import_module(pkg + ".console").Console
Table = importlib.import_module(pkg + ".table").Table
Panel = importlib.import_module(pkg + ".panel").Panel
Syntax = importlib.import_module(pkg + ".syntax").Syntax
def bench(name, make, n=5):
    best = 1e9
    for _ in range(n):
        c = Console(file=io.StringIO(), width=100, force_terminal=True, color_system="truecolor")
        obj = make()
        t = time.perf_counter(); c.print(obj); best = min(best, time.perf_counter() - t)
    print(f"{pkg:10} {name:28} {best*1000:8.1f} ms")
def table():
    t = Table(title="T")
    for h in "ABCDE": t.add_column(h)
    for i in range(1000): t.add_row(str(i), "[bold]x[/] y", "lorem ipsum dolor", f"{i*3.1:.2f}", "z")
    return t
bench("table 1000x5", table)
bench("panel of long text", lambda: Panel("word " * 20000))
code = open(importlib.import_module(pkg + ".table").__file__).read()
bench("syntax (table.py source)", lambda: Syntax(code, "python", line_numbers=True), n=3)
bench("1000 markup prints", lambda: __import__(pkg+".console", fromlist=["Group"]).Group(*[f"[red]line[/] {i} [b]bold[/]" for i in range(1000)]))
