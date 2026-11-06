# Verification & Troubleshooting

Ensure `fast_rich` is installed and working correctly in your environment.

## 1. Verify Installation

Check the installed version and importability:

```bash
python -c "import fast_rich; print(f'Fast Rich v{fast_rich.__version__} installed successfully!')"
```

## 2. Visual Verification Script

Create a file named `verify_fast_rich.py` with the following content to test color output and formatting:

```python
from fast_rich import print
from fast_rich.console import Console
from fast_rich.panel import Panel

print("[bold magenta]Fast Rich Verification[/bold magenta]")
print("If you see this styled, basic printing works!")

console = Console()
console.rule("[bold red]Console Rule[/]")

panel = Panel("[green]Everything is working correctly![/green]", title="Status", border_style="blue")
console.print(panel)
```

Run it:

```bash
python verify_fast_rich.py
```

You should see colored output, a rule line, and a panel with a blue border.

## 3. Troubleshooting

### Output is Plain Text (No Color)

If you see raw tags like `[bold magenta]...[/bold magenta]` instead of styled text:

1.  **Check Terminal Support**: Ensure your terminal supports ANSI colors.
2.  **Force Terminal Mode**: If running in a script or CI where a TTY isn't detected, force terminal mode:

    ```python
    from fast_rich.console import Console
    console = Console(force_terminal=True)
    console.print("[bold red]Forced Color[/]")
    ```

### "No module named imports"

If you get `ImportError` regarding `_core`:

1.  Reinstall the package:
    ```bash
    pip install --force-reinstall fast-rich
    ```
2.  Ensure you are using a compatible Python version (3.10+).

### Performance Issues

If performance isn't as expected (e.g., similar to `rich`):

1. Confirm you are using `fast_rich` imports, not `rich`.
2. Ensure the Rust extension is loaded. You can check:
   ```python
   import fast_rich.console
   c = fast_rich.console.Console()
   print(f"Using Rust Core: {c._use_rust}")
   ```
   It should print `True`.
