import time
import rich_rust
from rich.syntax import Syntax as RichSyntax
from rich.console import Console as RichConsole

ITERATIONS = 500
CODE = """
fn main() {
    let x = 42;
    println!("Hello World {}", x);
    if x > 10 {
        // Comment
        vec![1, 2, 3].iter().map(|i| i * 2).collect::<Vec<_>>();
    }
}
""" * 50

def bench_python_syntax():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        syn = RichSyntax(CODE, "rust", theme="monokai", line_numbers=True)
        console.print(syn)

def bench_rust_syntax():
    console = rich_rust.Console()
    for _ in range(ITERATIONS):
        syn = rich_rust.Syntax(CODE, "rust", theme="monokai", line_numbers=True)
        console.print_syntax(syn)

if __name__ == "__main__":
    start = time.time()
    bench_python_syntax()
    print(f"Python Syntax: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_syntax()
    print(f"Rust Syntax:   {time.time() - start:.4f}s")
