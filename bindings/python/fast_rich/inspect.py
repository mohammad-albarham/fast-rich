"""Inspect utility - matches rich.inspect API."""

from __future__ import annotations

import inspect as python_inspect
from typing import Any, Optional

from fast_rich.console import Console


def inspect(
    obj: Any,
    *,
    console: Optional[Console] = None,
    title: Optional[str] = None,
    help: bool = False,
    methods: bool = False,
    docs: bool = True,
    private: bool = False,
    dunder: bool = False,
    all: bool = False,
    sort: bool = True,
    value: bool = True,
) -> None:
    """Inspect an object.
    
    Matches rich.inspect API.
    
    Args:
        obj: Object to inspect.
        console: Console to use.
        title: Custom title.
        help: Show help text.
        methods: Show methods.
        docs: Show docstrings.
        private: Show private members.
        dunder: Show dunder members.
        all: Show all members.
        sort: Sort members.
        value: Show values.
    """
    _console = console or Console()
    
    # Get object info
    obj_type = type(obj).__name__
    obj_title = title or f"<{obj_type}>"
    
    _console.print(f"╭─ {obj_title} ─╮")
    
    # Show value
    if value:
        try:
            val_str = repr(obj)
            if len(val_str) > 80:
                val_str = val_str[:77] + "..."
            _console.print(f"│ Value: {val_str}")
        except Exception:
            pass
    
    # Show docstring
    if docs and obj.__doc__:
        doc = obj.__doc__.strip().split("\n")[0]
        if len(doc) > 70:
            doc = doc[:67] + "..."
        _console.print(f"│ Doc: {doc}")
    
    # Get members
    members = []
    try:
        for name in dir(obj):
            # Filter based on options
            if name.startswith("__") and name.endswith("__"):
                if not (dunder or all):
                    continue
            elif name.startswith("_"):
                if not (private or all):
                    continue
            
            try:
                attr = getattr(obj, name)
                is_method = callable(attr)
                
                if is_method and not (methods or all):
                    continue
                    
                members.append((name, attr, is_method))
            except Exception:
                continue
    except Exception:
        pass
    
    if sort:
        members.sort(key=lambda x: x[0])
    
    # Display members
    for name, attr, is_method in members:
        if is_method:
            try:
                sig = str(python_inspect.signature(attr))
            except (ValueError, TypeError):
                sig = "()"
            _console.print(f"│  {name}{sig}")
        else:
            try:
                val = repr(attr)
                if len(val) > 50:
                    val = val[:47] + "..."
                _console.print(f"│  {name} = {val}")
            except Exception:
                _console.print(f"│  {name}")
    
    _console.print("╰" + "─" * (len(obj_title) + 4) + "╯")


__all__ = ["inspect"]
