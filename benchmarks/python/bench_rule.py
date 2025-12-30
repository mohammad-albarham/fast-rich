import time
import rich_rust
from rich.console import Console as RichConsole
from rich.rule import Rule as RichRule
from rich.style import Style as RichStyle

ITERATIONS = 1000

def bench_python_rule():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        # Untitled rule
        console.print(RichRule())
        # Titled rule
        console.print(RichRule(title="Title"))
        # Styled rule
        console.print(RichRule(title="Styled", style="bold red"))

def bench_rust_rule():
    console = rich_rust.Console()
    style = rich_rust.Style(bold=True, color="red")
    for _ in range(ITERATIONS):
        # Untitled rule (assuming Rule.line() or empty constructor binding?)
        # Checking bindings... PyRule::new(title, style)
        # If title is None, it should be a line.
        console.print_rule(rich_rust.Rule()) 
        console.print_rule(rich_rust.Rule("Title"))
        console.print_rule(rich_rust.Rule("Styled", style))

if __name__ == "__main__":
    start = time.time()
    bench_python_rule()
    print(f"Python Rule: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_rule()
    print(f"Rust Rule:   {time.time() - start:.4f}s")
