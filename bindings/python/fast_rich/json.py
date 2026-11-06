"""JSON highlighting - matches rich.json API."""

from __future__ import annotations

import json as stdlib_json
from typing import Any, Optional, Union

from fast_rich.style import Style
from fast_rich.text import Text


class JSON:
    """A renderable that pretty prints JSON data.
    
    Matches rich.json.JSON API.
    """

    def __init__(
        self,
        json: str,
        *,
        indent: Union[int, str] = 2,
        highlight: bool = True,
        skip_keys: bool = False,
        ensure_ascii: bool = False,
        check_circular: bool = True,
        allow_nan: bool = True,
        default: Optional[Any] = None,
        sort_keys: bool = False,
    ) -> None:
        """Create JSON renderable.
        
        Args:
            json: JSON string.
            indent: Indentation.
            highlight: Syntax highlight.
            skip_keys: Skip non-string keys.
            ensure_ascii: Ensure ASCII output.
            check_circular: Check circular references.
            allow_nan: Allow NaN values.
            default: Default encoder.
            sort_keys: Sort keys.
        """
        self.json = json
        self.indent = indent
        self.highlight = highlight
        self.skip_keys = skip_keys
        self.ensure_ascii = ensure_ascii
        self.check_circular = check_circular
        self.allow_nan = allow_nan
        self.default = default
        self.sort_keys = sort_keys

    @classmethod
    def from_data(
        cls,
        data: Any,
        *,
        indent: Union[int, str] = 2,
        highlight: bool = True,
        skip_keys: bool = False,
        ensure_ascii: bool = False,
        check_circular: bool = True,
        allow_nan: bool = True,
        default: Optional[Any] = None,
        sort_keys: bool = False,
    ) -> "JSON":
        """Create JSON from Python data.
        
        Args:
            data: Python object to serialize.
            indent: Indentation.
            highlight: Syntax highlight.
            Other args: Passed to json.dumps.
            
        Returns:
            JSON instance.
        """
        json_str = stdlib_json.dumps(
            data,
            indent=indent,
            skipkeys=skip_keys,
            ensure_ascii=ensure_ascii,
            check_circular=check_circular,
            allow_nan=allow_nan,
            default=default,
            sort_keys=sort_keys,
        )
        return cls(
            json_str,
            indent=indent,
            highlight=highlight,
            skip_keys=skip_keys,
            ensure_ascii=ensure_ascii,
            check_circular=check_circular,
            allow_nan=allow_nan,
            default=default,
            sort_keys=sort_keys,
        )

    def __str__(self) -> str:
        """Render as string."""
        try:
            # Parse and re-format with proper indent
            data = stdlib_json.loads(self.json)
            indent = self.indent if isinstance(self.indent, int) else len(self.indent)
            return stdlib_json.dumps(
                data,
                indent=indent,
                skipkeys=self.skip_keys,
                ensure_ascii=self.ensure_ascii,
                sort_keys=self.sort_keys,
            )
        except Exception:
            return self.json

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield str(self)


__all__ = ["JSON"]
