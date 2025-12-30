import time
import rich_rust
from rich.text import Text as RichText
from rich.console import Console as RichConsole

ITERATIONS = 5000
MARKUP_STR = "[bold red]Hello[/] [blue]World[/]! " * 20

def bench_python_text():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        t = RichText.from_markup(MARKUP_STR)
        console.print(t)

def bench_rust_text():
    # Rust bindings Console prints to stdout, so we might want to capture/suppress for pure render bench?
    # For now, let's just construct.
    console_rust = rich_rust.Console()
    for _ in range(ITERATIONS):
        t = rich_rust.Text.from_markup(MARKUP_STR)
        console_rust.print_text(t)

if __name__ == "__main__":
    start = time.time()
    bench_python_text()
    print(f"Python Text: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_text()
    print(f"Rust Text:   {time.time() - start:.4f}s")
