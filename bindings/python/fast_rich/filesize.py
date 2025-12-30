"""File size formatting - matches rich._fileno_to_path/filesize API."""

from __future__ import annotations

from typing import Tuple


def decimal(size: int, *, precision: int = 1) -> str:
    """Convert size to human readable format (decimal).
    
    Uses SI units (KB, MB, GB).
    
    Args:
        size: Size in bytes.
        precision: Decimal precision.
        
    Returns:
        Human readable size string.
    """
    return _format_size(size, ("bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"), 1000, precision)


def traditional(size: int, *, precision: int = 1) -> str:
    """Convert size to human readable format (binary).
    
    Uses binary units (KiB, MiB, GiB).
    
    Args:
        size: Size in bytes.
        precision: Decimal precision.
        
    Returns:
        Human readable size string.
    """
    return _format_size(size, ("bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"), 1024, precision)


def _format_size(
    size: int,
    units: Tuple[str, ...],
    base: int,
    precision: int,
) -> str:
    """Format size with given units.
    
    Args:
        size: Size in bytes.
        units: Unit names.
        base: Base (1000 or 1024).
        precision: Decimal places.
        
    Returns:
        Formatted size string.
    """
    if size < 0:
        return f"-{_format_size(-size, units, base, precision)}"
    
    if size == 1:
        return "1 byte"
    if size < base:
        return f"{size} {units[0]}"
    
    for unit in units[1:]:
        size /= base
        if abs(size) < base:
            return f"{size:.{precision}f} {unit}"
    
    return f"{size:.{precision}f} {units[-1]}"


def pick_unit_and_suffix(
    size: int,
    suffixes: Tuple[str, ...],
    base: int,
) -> Tuple[float, str]:
    """Pick appropriate unit for size.
    
    Args:
        size: Size in bytes.
        suffixes: Unit suffixes.
        base: Base (1000 or 1024).
        
    Returns:
        Tuple of (value, suffix).
    """
    if size == 0:
        return 0, suffixes[0]
    
    for suffix in suffixes[:-1]:
        if abs(size) < base:
            return size, suffix
        size /= base
    
    return size, suffixes[-1]


__all__ = ["decimal", "traditional", "pick_unit_and_suffix"]
