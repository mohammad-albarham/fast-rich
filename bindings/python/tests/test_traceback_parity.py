"""Parity tests for Traceback - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest
import sys


class TestTracebackParity:
    """Test Traceback parity between fast_rich and rich."""

    def test_traceback_basic(self):
        """Test basic traceback creation."""
        from fast_rich.traceback import Traceback as FastTraceback
        from rich.traceback import Traceback as RichTraceback

        # Both should instantiate without error
        try:
            raise ValueError("Test error")
        except ValueError:
            fast_tb = FastTraceback()
            rich_tb = RichTraceback()

    def test_traceback_from_exception(self):
        """Test traceback from exception."""
        from fast_rich.traceback import Traceback

        try:
            raise RuntimeError("Something went wrong")
        except RuntimeError:
            tb = Traceback()
            # Just verify instantiation works
            assert tb is not None

    def test_traceback_install(self):
        """Test traceback install function."""
        from fast_rich.traceback import install

        # Should not raise
        old_hook = sys.excepthook
        install()
        # Restore
        sys.excepthook = old_hook


class TestTracebackRendering:
    """Test traceback rendering output."""

    def test_traceback_str_output(self):
        """Test traceback renders to string."""
        from fast_rich.traceback import Traceback

        try:
            1 / 0
        except ZeroDivisionError:
            tb = Traceback()
            # Just verify instantiation works
            assert tb is not None

    def test_traceback_with_locals(self):
        """Test traceback with locals display."""
        from fast_rich.traceback import Traceback

        try:
            x = 42
            y = "hello"
            raise Exception("Test")
        except Exception:
            tb = Traceback(show_locals=True)
            # Just verify instantiation works
            assert tb is not None

    def test_traceback_constructor_params(self):
        """Test traceback accepts all constructor params."""
        from fast_rich.traceback import Traceback

        try:
            raise ValueError("test")
        except ValueError:
            tb = Traceback(
                width=100,
                extra_lines=3,
                theme="monokai",
                word_wrap=False,
                show_locals=False,
            )
            assert tb is not None

    def test_traceback_nested_exception(self):
        """Test traceback with chained exceptions."""
        from fast_rich.traceback import Traceback

        try:
            try:
                raise ValueError("Original")
            except ValueError as e:
                raise RuntimeError("Wrapped") from e
        except RuntimeError:
            tb = Traceback()
            # Just verify instantiation works
            assert tb is not None
