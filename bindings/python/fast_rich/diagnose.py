"""Diagnostics - matches rich.diagnose API."""

from __future__ import annotations

import os
import platform
import sys
from typing import Optional

from fast_rich.console import Console


def report() -> None:
    """Print diagnostic information about the console."""
    console = Console()
    
    console.print("[bold]Rich Diagnostics[/]")
    console.print()
    
    # Python info
    console.rule("Python")
    console.print(f"Python: {sys.version}")
    console.print(f"Platform: {platform.platform()}")
    console.print(f"Executable: {sys.executable}")
    
    # Environment
    console.rule("Environment")
    console.print(f"TERM: {os.environ.get('TERM', 'not set')}")
    console.print(f"COLORTERM: {os.environ.get('COLORTERM', 'not set')}")
    console.print(f"FORCE_COLOR: {os.environ.get('FORCE_COLOR', 'not set')}")
    console.print(f"NO_COLOR: {os.environ.get('NO_COLOR', 'not set')}")
    
    # Console info
    console.rule("Console")
    console.print(f"Width: {console.width}")
    console.print(f"Height: {console.height}")
    console.print(f"Is Terminal: {console.is_terminal}")
    console.print(f"Color System: {console.color_system}")
    console.print(f"Encoding: {console.encoding}")
    
    # Features
    console.rule("Features")
    console.print("[bold green]✓[/] Console output")
    console.print("[bold green]✓[/] Style support")
    console.print("[bold green]✓[/] Markup support")
    console.print("[bold green]✓[/] Table support")
    console.print("[bold green]✓[/] Progress bars")
    
    console.print()
    console.print("[dim]fast_rich powered by Rust[/]")


__all__ = ["report"]
