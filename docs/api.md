# Rust API

The full API documentation for the Rust crate is available via standard `cargo doc`.

## Running locally

```bash
cargo doc --open
```

## Main Modules

*   [`console`](https://docs.rs/rich-rust/latest/rich_rust/console/index.html)
*   [`table`](https://docs.rs/rich-rust/latest/rich_rust/table/index.html)
*   [`progress`](https://docs.rs/rich-rust/latest/rich_rust/progress/index.html)
*   [`style`](https://docs.rs/rich-rust/latest/rich_rust/style/index.html)

## Example

```rust
use rich_rust::prelude::*;

let mut table = Table::new();
table.add_column("Col 1");
table.add_row(vec!["Cell 1"]);

Console::new().print_renderable(&table);
```
