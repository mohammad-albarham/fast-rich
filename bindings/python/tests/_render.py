"""Render a fast_rich renderable to plain text, the way Rich users do."""

import io


def render(renderable, width: int = 80) -> str:
    from fast_rich.console import Console

    console = Console(file=io.StringIO(), width=width, color_system=None)
    console.print(renderable)
    return console.file.getvalue()
