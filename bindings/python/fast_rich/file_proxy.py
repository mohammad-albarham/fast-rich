"""File proxy - matches rich.file_proxy API."""

from __future__ import annotations

import sys
from typing import IO, Any, Optional


class FileProxy:
    """Proxy for file-like objects.
    
    Matches rich.file_proxy.FileProxy API.
    """

    def __init__(
        self,
        file: IO[str],
        *,
        console: Optional[Any] = None,
    ) -> None:
        """Create FileProxy.
        
        Args:
            file: File to proxy.
            console: Console for output.
        """
        self._file = file
        self._console = console

    @property
    def rich_proxied_file(self) -> IO[str]:
        """Get the proxied file."""
        return self._file

    def write(self, text: str) -> int:
        """Write text to file.
        
        Args:
            text: Text to write.
            
        Returns:
            Number of characters written.
        """
        if self._console is not None:
            self._console.print(text, end="")
            return len(text)
        return self._file.write(text)

    def flush(self) -> None:
        """Flush the file."""
        self._file.flush()

    def fileno(self) -> int:
        """Get file descriptor."""
        return self._file.fileno()

    @property
    def mode(self) -> str:
        """Get file mode."""
        return getattr(self._file, "mode", "w")

    @property
    def name(self) -> str:
        """Get file name."""
        return getattr(self._file, "name", "<unknown>")

    @property
    def encoding(self) -> str:
        """Get encoding."""
        return getattr(self._file, "encoding", "utf-8")

    def isatty(self) -> bool:
        """Check if file is a TTY."""
        return hasattr(self._file, "isatty") and self._file.isatty()


__all__ = ["FileProxy"]
