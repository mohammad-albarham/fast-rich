# Contributing to fast-rich

Thank you for your interest in contributing to `fast-rich`! We welcome contributions of all forms, including bug reports, feature requests, documentation improvements, and code changes.

## Development Setup

1.  **Prerequisites**:
    *   Rust (stable)
    *   Python 3.8+ (for bindings)
    *   `maturin` (for building python bindings: `pip install maturin`)

2.  **Clone the repository**:
    ```bash
    git clone https://github.com/mohammad-albarham/fast-rich
    cd fast-rich
    ```

3.  **Run Tests**:
    ```bash
    cargo test
    ```

4.  **Run Examples**:
    ```bash
    cargo run --example showcase
    ```

## Python Bindings Development

The Python bindings are located in `bindings/python`.

1.  **Build and Install (Dev Mode)**:
    ```bash
    maturin develop
    ```

2.  **Run Python Tests**:
    ```bash
    pytest
    ```

## Coding Standards

*   **Formatting**: We use `rustfmt`. Please run `cargo fmt` before submitting.
*   **Linting**: We use `clippy`. Please run `cargo clippy` and address warnings.
*   **Workflow**: Use the `scripts/auto_commit.sh` script if available to automatically format, test, and commit standard changes.

## Pull Request Process

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feat/my-feature`).
3.  Commit your changes.
4.  Push to the branch.
5.  Open a Pull Request.

## Documentation

Documentation is built with `mkdocs`.
```bash
mkdocs serve
```
