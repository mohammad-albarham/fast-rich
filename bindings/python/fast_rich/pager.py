"""Pager support - matches rich.pager API."""

from __future__ import annotations

import os
import subprocess
import sys
from typing import Optional


class Pager:
    """A pager for displaying long content.
    
    Matches rich.pager.Pager API.
    """

    def __init__(self, command: Optional[str] = None) -> None:
        """Create Pager.
        
        Args:
            command: Pager command (default: uses PAGER env or less/more).
        """
        self.command = command

    def _get_pager_command(self) -> str:
        """Get the pager command to use."""
        if self.command:
            return self.command
        
        pager = os.environ.get("PAGER", "")
        if pager:
            return pager
        
        # Try common pagers
        for cmd in ["less", "more"]:
            try:
                subprocess.run([cmd, "--version"], capture_output=True)
                return cmd
            except FileNotFoundError:
                continue
        
        return "cat"

    def show(self, content: str) -> None:
        """Show content in pager.
        
        Args:
            content: Content to display.
        """
        pager_cmd = self._get_pager_command()
        
        try:
            process = subprocess.Popen(
                pager_cmd,
                shell=True,
                stdin=subprocess.PIPE,
                text=True,
            )
            process.communicate(input=content)
        except Exception:
            # Fallback to print
            print(content)


class SystemPager(Pager):
    """System pager using PAGER environment variable."""
    pass


__all__ = ["Pager", "SystemPager"]
