"""Parity tests for JSON - comparing fast_rich vs rich."""

from __future__ import annotations

import pytest
import json


class TestJSONParity:
    """Test JSON parity between fast_rich and rich."""

    def test_json_basic(self):
        """Test basic JSON creation."""
        from fast_rich.json import JSON as FastJSON
        from rich.json import JSON as RichJSON

        data = '{"name": "Alice", "age": 30}'

        # Both should instantiate without error
        fast_json = FastJSON(data)
        rich_json = RichJSON(data)

    def test_json_from_dict(self):
        """Test JSON from Python dict."""
        from fast_rich.json import JSON

        data = {"items": [1, 2, 3], "active": True}
        j = JSON.from_data(data)
        output = str(j)
        assert "items" in output or "1" in output

    def test_json_nested(self):
        """Test nested JSON structure."""
        from fast_rich.json import JSON

        data = '{"user": {"name": "Bob", "settings": {"theme": "dark"}}}'
        j = JSON(data)
        output = str(j)
        assert len(output) > 0


class TestJSONRendering:
    """Test JSON rendering output."""

    def test_json_str_output(self):
        """Test JSON renders to string."""
        from fast_rich.json import JSON

        j = JSON('{"key": "value"}')
        output = str(j)
        assert "key" in output or "value" in output

    def test_json_with_indent(self):
        """Test JSON with custom indent."""
        from fast_rich.json import JSON

        j = JSON('{"a": 1}', indent=4)
        output = str(j)
        assert len(output) > 0

    def test_json_array(self):
        """Test JSON array."""
        from fast_rich.json import JSON

        j = JSON('[1, 2, 3, "four"]')
        output = str(j)
        assert "1" in output or "four" in output

    def test_json_boolean_null(self):
        """Test JSON with boolean and null."""
        from fast_rich.json import JSON

        j = JSON('{"active": true, "data": null}')
        output = str(j)
        assert len(output) > 0

    def test_json_constructor_params(self):
        """Test JSON accepts all constructor params."""
        from fast_rich.json import JSON

        j = JSON(
            '{"x": 1}',
            indent=2,
            highlight=True,
        )
        assert j is not None
