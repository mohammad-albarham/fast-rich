import time
import rich_rust
from rich.markdown import Markdown as RichMarkdown
from rich.console import Console as RichConsole

ITERATIONS = 500
MARKDOWN_CONTENT = """
# Heading

## Subheading

* List item 1
* List item 2

```rust
fn main() {
    println!("Hello");
}
```

Paragraph with **bold** and *italic* text.
""" * 10

def bench_python_markdown():
    console = RichConsole(file=open('/dev/null', 'w'))
    for _ in range(ITERATIONS):
        md = RichMarkdown(MARKDOWN_CONTENT)
        console.print(md)

def bench_rust_markdown():
    console = rich_rust.Console()
    for _ in range(ITERATIONS):
        md = rich_rust.Markdown(MARKDOWN_CONTENT)
        console.print_markdown(md)

if __name__ == "__main__":
    start = time.time()
    bench_python_markdown()
    print(f"Python Markdown: {time.time() - start:.4f}s")
    
    start = time.time()
    bench_rust_markdown()
    print(f"Rust Markdown:   {time.time() - start:.4f}s")
