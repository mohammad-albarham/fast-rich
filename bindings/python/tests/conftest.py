"""Pytest configuration and fixtures."""

from __future__ import annotations

import pytest


@pytest.fixture
def fast_console():
    """Create a fast_rich Console with in-memory output."""
    import io
    from fast_rich.console import Console
    output = io.StringIO()
    console = Console(file=output)
    console._output = output  # Store reference
    return console


@pytest.fixture
def rich_console():
    """Create a rich Console with in-memory output."""
    import io
    from rich.console import Console
    output = io.StringIO()
    console = Console(file=output, force_terminal=True)
    console._output = output  # Store reference
    return console
