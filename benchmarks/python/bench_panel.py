import time
import rich_rust
from rich.panel import Panel as RichPanel
from rich.console import Console as RichConsole
from rich.text import Text as RichText

ITERATIONS = 5000
TEXT_CONTENT = "Hello World" * 10
TITLE = "My Panel"

def bench_python_panel():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        p = RichPanel(TEXT_CONTENT, title=TITLE)
        console.print(p)

def bench_rust_panel():
    console = rich_rust.Console()
    for _ in range(ITERATIONS):
        # We need to construct Text first for our binding
        t = rich_rust.Text(TEXT_CONTENT) 
        p = rich_rust.Panel(t, title=TITLE)
        console.print_panel(p)

if __name__ == "__main__":
    start = time.time()
    bench_python_panel()
    print(f"Python Panel: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_panel()
    print(f"Rust Panel:   {time.time() - start:.4f}s")
