"""Terminal utilities - matches rich.terminal_theme and terminal APIs."""

from __future__ import annotations

from typing import Dict, Tuple

from fast_rich.color import ColorTriplet


class TerminalTheme:
    """A theme for terminal colors.
    
    Matches rich.terminal_theme.TerminalTheme API.
    """

    def __init__(
        self,
        background: Tuple[int, int, int],
        foreground: Tuple[int, int, int],
        normal: Tuple[
            Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int],
            Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int],
        ],
        bright: Tuple[
            Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int],
            Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int], Tuple[int, int, int],
        ],
    ) -> None:
        """Create TerminalTheme.
        
        Args:
            background: Background color.
            foreground: Foreground color.
            normal: Normal colors (black, red, green, yellow, blue, magenta, cyan, white).
            bright: Bright colors.
        """
        self.background = ColorTriplet(*background)
        self.foreground = ColorTriplet(*foreground)
        self.ansi_colors = {
            i: ColorTriplet(*normal[i]) for i in range(8)
        }
        self.ansi_colors.update({
            i + 8: ColorTriplet(*bright[i]) for i in range(8)
        })


# Default terminal themes
MONOKAI = TerminalTheme(
    (39, 40, 34),
    (248, 248, 242),
    (
        (0, 0, 0), (249, 38, 114), (166, 226, 46), (244, 191, 117),
        (102, 217, 239), (174, 129, 255), (161, 239, 228), (248, 248, 242),
    ),
    (
        (117, 113, 94), (255, 84, 168), (196, 255, 94), (255, 231, 146),
        (134, 255, 255), (199, 159, 252), (190, 255, 242), (255, 255, 255),
    ),
)

DIMMED_MONOKAI = TerminalTheme(
    (25, 25, 25),
    (185, 186, 181),
    (
        (0, 0, 0), (197, 15, 31), (122, 179, 43), (180, 141, 64),
        (65, 154, 207), (126, 87, 194), (83, 180, 173), (171, 171, 162),
    ),
    (
        (104, 104, 104), (215, 95, 107), (157, 205, 105), (214, 191, 110),
        (111, 189, 239), (161, 123, 218), (131, 210, 204), (244, 244, 244),
    ),
)

NIGHT_OWLISH = TerminalTheme(
    (11, 22, 38),
    (171, 180, 194),
    (
        (1, 22, 39), (255, 91, 102), (101, 222, 134), (255, 203, 139),
        (130, 170, 255), (199, 146, 234), (128, 203, 196), (171, 180, 194),
    ),
    (
        (87, 108, 138), (255, 100, 94), (68, 232, 161), (255, 208, 102),
        (158, 190, 255), (199, 146, 234), (111, 220, 196), (255, 255, 255),
    ),
)

SVG_EXPORT_THEME = MONOKAI


__all__ = [
    "TerminalTheme",
    "MONOKAI",
    "DIMMED_MONOKAI",
    "NIGHT_OWLISH",
    "SVG_EXPORT_THEME",
]
