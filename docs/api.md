# Rust API

The full API documentation for the Rust crate is available via standard `cargo doc`.

## Running locally

```bash
cargo doc --open
```

## Main Modules

*   [`console`](https://docs.rs/fast-rich/latest/rich_rust/console/index.html)
*   [`table`](https://docs.rs/fast-rich/latest/rich_rust/table/index.html)
*   [`progress`](https://docs.rs/fast-rich/latest/rich_rust/progress/index.html)
*   [`style`](https://docs.rs/fast-rich/latest/rich_rust/style/index.html)
*   [`live`](https://docs.rs/fast-rich/latest/rich_rust/live/index.html)
*   [`syntax`](https://docs.rs/fast-rich/latest/rich_rust/syntax/index.html)
*   [`log`](https://docs.rs/fast-rich/latest/rich_rust/log/index.html)
*   [`layout`](https://docs.rs/fast-rich/latest/rich_rust/layout/index.html)

## Example

```rust
use rich_rust::prelude::*;

let mut table = Table::new();
table.add_column("Col 1");
table.add_row(vec!["Cell 1"]);

Console::new().print_renderable(&table);
```
