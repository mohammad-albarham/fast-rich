# Contributing to Fast-Rich

Thank you for your interest in contributing to Fast-Rich! This guide covers development setup, coding standards, and the contribution process.

## Development Setup

### Prerequisites

- **Rust** (stable, 1.70+)
- **Python 3.8+** (for bindings development)
- **maturin** (for Python bindings): `pip install maturin`

### Clone and Build

```bash
git clone https://github.com/mohammad-albarham/fast-rich
cd fast-rich
cargo build
```

### Run Tests

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run a specific test
cargo test test_name
```

### Run Examples

```bash
cargo run --example showcase --features full
cargo run --example tables_demo
cargo run --example progress_bar
```

---

## Project Structure

```
fast-rich/
├── src/                    # Rust library source
│   ├── lib.rs             # Main library entry
│   ├── console.rs         # Console output
│   ├── style.rs           # Colors and styles
│   ├── table.rs           # Tables
│   ├── progress/          # Progress bars
│   └── ...
├── examples/              # Example programs
├── tests/                 # Integration tests
├── bindings/              # Python bindings
├── docs/                  # Documentation (this site)
└── benches/               # Benchmarks
```

---

## Coding Standards

### Formatting

We use `rustfmt`. Always format before committing:

```bash
cargo fmt
```

### Linting

We use `clippy`. Address all warnings:

```bash
cargo clippy --all-features
```

### Documentation

- All public items must have doc comments
- Include examples in doc comments where helpful
- Keep doc comments concise but complete

```rust
/// Creates a new styled text span.
///
/// # Arguments
///
/// * `content` - The text content
/// * `style` - The style to apply
///
/// # Example
///
/// ```
/// let text = Text::styled("Hello", Style::new().bold());
/// ```
pub fn styled(content: &str, style: Style) -> Self {
    // ...
}
```

---

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** for your feature or fix:
   ```bash
   git checkout -b feat/my-feature
   # or
   git checkout -b fix/my-bugfix
   ```
3. **Make your changes** with clear, atomic commits
4. **Run checks**:
   ```bash
   cargo fmt
   cargo clippy --all-features
   cargo test --all-features
   ```
5. **Push** and open a Pull Request

### Commit Convention

Use conventional commits:

- `feat: add new table border style`
- `fix: correct color parsing for RGB`
- `docs: update progress bar guide`
- `test: add tests for layout splitting`
- `refactor: simplify console output logic`

---

## Python Bindings

The Python bindings are in `bindings/python`.

### Build and Test

```bash
cd bindings/python
maturin develop
pytest
```

### Structure

```
bindings/python/
├── src/
│   └── lib.rs        # PyO3 bindings
├── python/
│   └── fast_rich/    # Python package
└── tests/
    └── test_*.py     # Python tests
```

---

## Documentation

### Building Docs

This documentation uses MkDocs with Material theme.

```bash
# Install dependencies
pip install mkdocs-material pymdown-extensions

# Serve locally
mkdocs serve

# Build static site
mkdocs build
```

### Documentation Standards

- Use clear, concise prose
- Include code examples for every feature
- Show real terminal output where possible
- Use admonitions (tips, warnings, notes) sparingly
- Keep the structure consistent across pages

See [Building Docs](building-docs.md) for more details.

---

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // ...
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
// tests/test_tables.rs
use fast_rich::prelude::*;

#[test]
fn test_table_rendering() {
    let console = Console::capture();
    // ...
    let output = console.get_captured_output();
    assert!(output.contains("expected"));
}
```

### ANSI Output Testing

For ANSI escape code testing, use byte-level assertions:

```rust
#[test]
fn test_ansi_output() {
    let console = Console::capture();
    console.print("[red]Hello[/]");
    let output = console.get_captured_output();
    
    // Check for ANSI red escape code
    assert!(output.contains("\x1b[31m"));
}
```

---

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/mohammad-albarham/fast-rich/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mohammad-albarham/fast-rich/discussions)

We appreciate all contributions, from bug reports to major features!
