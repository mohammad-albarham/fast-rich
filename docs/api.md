# Rust API

The full API documentation for the Rust crate is available via standard `cargo doc`.

## Running locally

```bash
cargo doc --open
```

## Main Modules

*   [`console`](https://docs.rs/fast-rich/latest/fast_rich/console/index.html)
*   [`table`](https://docs.rs/fast-rich/latest/fast_rich/table/index.html)
*   [`progress`](https://docs.rs/fast-rich/latest/fast_rich/progress/index.html)
*   [`style`](https://docs.rs/fast-rich/latest/fast_rich/style/index.html)
*   [`live`](https://docs.rs/fast-rich/latest/fast_rich/live/index.html)
*   [`syntax`](https://docs.rs/fast-rich/latest/fast_rich/syntax/index.html)
*   [`log`](https://docs.rs/fast-rich/latest/fast_rich/log/index.html)
*   [`layout`](https://docs.rs/fast-rich/latest/fast_rich/layout/index.html)

## Example

```rust
use fast_rich::prelude::*;

let mut table = Table::new();
table.add_column("Col 1");
table.add_row(vec!["Cell 1"]);

Console::new().print_renderable(&table);
```
