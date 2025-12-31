#!/usr/bin/env python3
"""Python rich reference script for ANSI comparison."""

from rich.console import Console

def main():
    console = Console(force_terminal=True, width=80)
    
    # Test 1: Basic markup with spaces
    console.print("Check [bold cyan]export.html[/] and [bold magenta]export.svg[/] files!")
    
    # Test 2: Emoji with text
    console.print(":rocket: [bold green]Launching[/] the application... :sparkles:")
    
    # Test 3: Mixed styles
    console.print(":warning: [bold yellow]Warning:[/] This is a [italic]test[/] message")
    
    # Test 4: Multiple styled spans
    console.print("[red]red[/] [blue]blue[/] [green]green[/]")

if __name__ == "__main__":
    main()
