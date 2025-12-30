import time
import sys
import rich_rust
from rich.console import Console as RichConsole
from rich.table import Table as RichTable
from rich.style import Style as RichStyle

# Setup
ROWS = 1000
COLS = 5

def bench_python_rich():
    console = RichConsole(file=open('/dev/null', 'w'))
    table = RichTable(title="Benchmark Table")
    
    for i in range(COLS):
        table.add_column(f"Col {i}")
        
    for i in range(ROWS):
        table.add_row(*[f"Cell {i},{j}" for j in range(COLS)])
        
    console.print(table)

def bench_rust_rich():
    console = rich_rust.Console() 
    # Current bindings don't support custom writers comfortably yet, 
    # but let's assume we want to measure object construction + rendering mostly.
    
    table = rich_rust.Table()
    
    for i in range(COLS):
        table.add_column(f"Col {i}")
        
    row_data = []
    for i in range(ROWS):
        row = [f"Cell {i},{j}" for j in range(COLS)]
        table.add_row(row)
    
    console.print_table(table)

if __name__ == "__main__":
    start = time.time()
    bench_python_rich()
    print(f"Python Rich: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_rich()
    print(f"Rust Rich:   {time.time() - start:.4f}s")
