use fast_rich::console::Console;
use fast_rich::markdown::Markdown;

fn main() {
    let console = Console::new();

    let md_source = r#"
# Markdown Parity Demo

This example demonstrates the **fast-rich** markdown renderer, striving for parity with Python's *rich*.

## Features

1. **Syntax Highlighting**:
   
   ```rust
   fn main() {
       println!("Hello from Rust!");
       let x = vec![1, 2, 3];
   }
   ```

   ```python
   def hello():
       print("Hello from Python!")
       x = [1, 2, 3]
   ```

2. **Tables**:

| Feature | Status | Notes |
|---------|--------|-------|
| Syntax  | ✅ Done | Using syntect |
| Tables  | ✅ Done | Rounded borders |
| Quotes  | ✅ Done | Styled margin |

3. **Blockquotes**:

> This is a blockquote.
> It can span multiple lines.
>
> And have paragraphs.

4. **Lists**:

- Item 1
- Item 2
    1. Nested Item 1
    2. Nested Item 2

---

## Conclusion

Markdown rendering is *essential* for a modern CLI experience.
"#;

    console.println_raw("\n");

    let md = Markdown::new(md_source);
    console.print_renderable(&md);

    console.println_raw("\n");
}
