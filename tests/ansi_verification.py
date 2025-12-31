#!/usr/bin/env python3
"""
ANSI Verification Script - Python Rich Reference
Generates output with raw ANSI codes for comparison with Rust implementation.
"""

import sys
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.text import Text
from rich.style import Style
from rich.align import Align

def capture_ansi(renderable, width=60):
    """Capture raw ANSI output from a renderable."""
    console = Console(width=width, force_terminal=True, legacy_windows=False, record=True)
    console.print(renderable)
    # Export with ANSI codes preserved
    return console.export_text(clear=False, styles="ansi")

def test_basic_styles():
    """Test basic text styles."""
    print("=== BASIC STYLES ===")
    text = "[bold]Bold[/] [italic]Italic[/] [underline]Underline[/]"
    output = capture_ansi(text)
    print(repr(output))
    return output

def test_colors():
    """Test color output."""
    print("\n=== COLORS ===")
    text = "[red]Red[/] [green]Green[/] [blue]Blue[/] [rgb(255,128,0)]Orange[/]"
    output = capture_ansi(text)
    print(repr(output))
    return output

def test_table():
    """Test table rendering."""
    print("\n=== TABLE ===")
    table = Table(title="Test Table")
    table.add_column("Language", style="cyan")
    table.add_column("Features", style="magenta")
    table.add_row("Rust", "Fast & Safe")
    table.add_row("Python", "Easy & Rich")
    output = capture_ansi(table)
    print(repr(output))
    return output

def test_panel():
    """Test panel rendering."""
    print("\n=== PANEL ===")
    panel = Panel("Hello, [bold green]World[/]!", title="Greeting")
    output = capture_ansi(panel)
    print(repr(output))
    return output

def test_align():
    """Test alignment."""
    print("\n=== ALIGN ===")
    text = Text("Centered Text", style="bold green")
    aligned = Align.center(text)
    output = capture_ansi(aligned)
    print(repr(output))
    return output

def test_padding():
    """Test padding wrapper."""
    print("\n=== PADDING ===")
    from rich.padding import Padding
    text = Text("Padded", style="cyan")
    padded = Padding(text, (1, 2))  # (vertical, horizontal)
    output = capture_ansi(padded)
    print(repr(output))
    return output

def test_bar_chart():
    """Test bar chart (not in Rich, so we'll skip for now)."""
    print("\n=== BAR (Skipped - not in Python Rich) ===")
    return ""

def test_theme():
    """Test theme colors."""
    print("\n=== THEME ===")
    from rich.theme import Theme
    from rich.console import Console as RichConsole
    
    theme = Theme({"success": "green", "warning": "yellow", "error": "red"})
    console = RichConsole(theme=theme, width=60, force_terminal=True, record=True)
    console.print("[success]Success[/success] [warning]Warning[/warning] [error]Error[/error]")
    output = console.export_text(clear=False, styles="ansi")
    print(repr(output))
    return output

def export_raw_bytes(test_name, output):
    """Export raw bytes to file for comparison."""
    filename = f"tests/ansi_output/python_{test_name}.txt"
    with open(filename, 'wb') as f:
        f.write(output.encode('utf-8'))
    print(f"Exported: {filename}")

if __name__ == "__main__":
    import os
    os.makedirs("tests/ansi_output", exist_ok=True)
    
    tests = [
        ("basic_styles", test_basic_styles),
        ("colors", test_colors),
        ("table", test_table),
        ("panel", test_panel),
        ("align", test_align),
        ("padding", test_padding),
        ("theme", test_theme),
    ]
    
    for name, test_func in tests:
        try:
            output = test_func()
            if output:  # Skip empty outputs
                export_raw_bytes(name, output)
        except Exception as e:
            print(f"Error in {name}: {e}")
    
    print("\n✓ All Python reference outputs generated")
