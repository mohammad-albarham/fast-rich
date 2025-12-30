import time
import rich_rust
from rich.console import Console as RichConsole
from rich.columns import Columns as RichColumns
from rich.text import Text as RichText

ITERATIONS = 1000
ITEMS = 50

def bench_python_columns():
    console = RichConsole(file=open('/dev/null', 'w'))
    items = [RichText(f"Item {i}", style="bold blue") for i in range(ITEMS)]
    for _ in range(ITERATIONS):
        console.print(RichColumns(items))

def bench_rust_columns():
    console = rich_rust.Console() 
    # Need to verify if we can construct PyText efficiently in loop or reuse?
    # bindings take Vec<PyText>. 
    style = rich_rust.Style(bold=True, color="blue")
    # Pre-create items to measure render time mostly? 
    # But Rich bench creates them. Let's create them.
    # We need PyText. from_plain or new.
    # rich_rust.Text("Item i", style)
    
    # Actually, let's pre-create the list to benchmark *rendering* primarily, 
    # similar to others if possible, or include construction if we want full picture.
    # Rich bench includes construction.
    
    items = [rich_rust.Text(f"Item {i}", style) for i in range(ITEMS)]
    
    for _ in range(ITERATIONS):
        # We need to pass fresh items or clone? 
        # Columns stores them.
        # If we pass `items` list, PyO3 converts it. 
        # But PyText is opaque wrapper. 
        # Hopefully cloning is cheap or handled by PyO3.
        console.print_columns(rich_rust.Columns(items))

if __name__ == "__main__":
    start = time.time()
    bench_python_columns()
    print(f"Python Columns: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_columns()
    print(f"Rust Columns:   {time.time() - start:.4f}s")
