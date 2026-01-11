# Print Macros

Fast-Rich provides drop-in replacement macros for the standard library's `print!` and `println!`, automatically adding rich formatting support.

## Shadowing Standard Macros

Simply import the macros to shadow the standard versions:

```rust
use fast_rich::{print, println};

fn main() {
    // Works exactly like std::println!, but with rich markup
    println!("[bold green]Success![/] Operation completed.");
    
    // Format arguments work as expected
    let count = 42;
    println!("Processed [cyan]{}[/] items", count);
    
    // Named arguments supported
    println!("{name} scored [yellow]{score}[/] points", name="Alice", score=100);
}
```

---

## Available Macros

| Macro | Description |
|:------|:------------|
| `print!` | Styled output to stdout (no newline) |
| `println!` | Styled output to stdout with newline |
| `eprint!` | Styled output to stderr (no newline) |
| `eprintln!` | Styled output to stderr with newline |
| `print_raw!` | Raw output to stdout (no markup parsing) |
| `println_raw!` | Raw output to stdout with newline (no parsing) |
| `eprint_raw!` | Raw output to stderr (no parsing) |
| `eprintln_raw!` | Raw output to stderr with newline (no parsing) |

---

## Using Aliases

When you need both standard and rich printing in the same file:

```rust
use fast_rich::rprintln;  // Alias that doesn't shadow std

fn main() {
    std::println!("Standard: [bold]this is literal[/]");
    rprintln!("Rich: [bold]this is bold[/]");
}
```

---

## Raw Output for Untrusted Data

The markup parser is smart—it ignores brackets that don't match style syntax. However, if your data might accidentally form valid markup tags, use raw macros:

```rust
use fast_rich::{print, println_raw};

fn main() {
    // Most debug output works fine
    let data = vec![1, 2, 3];
    println!("[cyan]Data:[/] {:?}", data);  // Works!
    
    // But if data looks like a style tag...
    #[derive(Debug)]
    enum Status { Red }
    let status = vec![Status::Red];
    
    // ⚠️ Problem: {:?} outputs "[Red]" which is a valid color tag!
    // println!("{:?}", status);  // Would be interpreted as red styling
    
    // ✅ Solution: Use raw macro
    print!("[cyan]Status:[/] ");
    println_raw!("{:?}", status);  // Prints "[Red]" literally
}
```

!!! warning "Edge Case: Tag Collisions"
    This limitation affects data where debug output exactly matches Rich markup syntax (e.g., `[Red]`, `[bold]`). Use `println_raw!` when printing untrusted or debug data.

---

## Stderr Output

For error messages and diagnostics:

```rust
use fast_rich::{eprint, eprintln, eprintln_raw};

fn main() {
    eprintln!("[bold red]Error:[/] Connection failed");
    
    eprint!("[yellow]Warning:[/] ");
    eprintln!("Retry in 5 seconds");
    
    // Raw for debug data
    eprintln_raw!("Context: {:?}", get_error_context());
}
```

---

## Try the Examples

Explore the full capabilities of print macros by running the example:

```bash
cargo run --example print_shadowing
```

This demo covers:
- ✅ Basic styled output with format arguments
- ✅ Named and positional arguments
- ✅ Inline progress dots
- ✅ Handling debug output with brackets
- ✅ Raw output for edge cases
- ✅ Stderr macros (`eprint!`, `eprintln!`)
- ✅ Using aliases alongside std macros
