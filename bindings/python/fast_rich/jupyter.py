"""Jupyter support - matches rich.jupyter API."""

from __future__ import annotations

from typing import Any, Optional

from fast_rich.console import Console


def is_jupyter() -> bool:
    """Check if running in Jupyter environment.
    
    Returns:
        True if in Jupyter.
    """
    try:
        from IPython import get_ipython
        shell = get_ipython()
        if shell is None:
            return False
        return shell.__class__.__name__ in ("ZMQInteractiveShell", "Shell")
    except ImportError:
        return False


def get_jupyter_display_format() -> str:
    """Get the preferred display format for Jupyter.
    
    Returns:
        Format name ("html", "text").
    """
    return "html" if is_jupyter() else "text"


class JupyterMixin:
    """Mixin for Jupyter display support."""

    def _repr_html_(self) -> Optional[str]:
        """HTML representation for Jupyter."""
        if not is_jupyter():
            return None
        console = Console(force_terminal=True, record=True)
        console.print(self)
        return console.export_html(inline_styles=True)

    def _repr_mimebundle_(
        self,
        include: Optional[Any] = None,
        exclude: Optional[Any] = None,
    ) -> dict:
        """Rich display for Jupyter."""
        return {
            "text/html": self._repr_html_(),
            "text/plain": str(self),
        }


def display(renderable: Any, console: Optional[Console] = None) -> None:
    """Display a renderable in Jupyter.
    
    Args:
        renderable: Object to display.
        console: Console to use.
    """
    if is_jupyter():
        try:
            from IPython.display import display as ipython_display, HTML
            _console = console or Console(force_terminal=True, record=True)
            _console.print(renderable)
            ipython_display(HTML(_console.export_html(inline_styles=True)))
        except ImportError:
            print(renderable)
    else:
        _console = console or Console()
        _console.print(renderable)


__all__ = ["is_jupyter", "get_jupyter_display_format", "JupyterMixin", "display"]
