"""Spinner animations - matches rich.spinner API."""

from __future__ import annotations

import itertools
import time
from typing import Iterator, Optional, Union

from fast_rich.style import Style
from fast_rich.text import Text


# Spinner frame definitions matching rich.spinner
SPINNERS = {
    "dots": {
        "frames": ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        "interval": 80,
    },
    "dots2": {
        "frames": ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
        "interval": 80,
    },
    "dots3": {
        "frames": ["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
        "interval": 80,
    },
    "line": {
        "frames": ["-", "\\", "|", "/"],
        "interval": 130,
    },
    "line2": {
        "frames": ["⠂", "-", "–", "—", "–", "-"],
        "interval": 100,
    },
    "pipe": {
        "frames": ["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
        "interval": 100,
    },
    "simpleDots": {
        "frames": [".  ", ".. ", "...", "   "],
        "interval": 400,
    },
    "simpleDotsScrolling": {
        "frames": [".  ", ".. ", "...", " ..", "  .", "   "],
        "interval": 200,
    },
    "star": {
        "frames": ["✶", "✸", "✹", "✺", "✹", "✷"],
        "interval": 70,
    },
    "star2": {
        "frames": ["+", "x", "*"],
        "interval": 80,
    },
    "flip": {
        "frames": ["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
        "interval": 70,
    },
    "hamburger": {
        "frames": ["☱", "☲", "☴"],
        "interval": 100,
    },
    "growVertical": {
        "frames": ["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃"],
        "interval": 120,
    },
    "growHorizontal": {
        "frames": ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"],
        "interval": 120,
    },
    "balloon": {
        "frames": [" ", ".", "o", "O", "@", "*", " "],
        "interval": 140,
    },
    "balloon2": {
        "frames": [".", "o", "O", "°", "O", "o", "."],
        "interval": 120,
    },
    "noise": {
        "frames": ["▓", "▒", "░"],
        "interval": 100,
    },
    "bounce": {
        "frames": ["⠁", "⠂", "⠄", "⠂"],
        "interval": 120,
    },
    "boxBounce": {
        "frames": ["▖", "▘", "▝", "▗"],
        "interval": 120,
    },
    "boxBounce2": {
        "frames": ["▌", "▀", "▐", "▄"],
        "interval": 100,
    },
    "triangle": {
        "frames": ["◢", "◣", "◤", "◥"],
        "interval": 50,
    },
    "arc": {
        "frames": ["◜", "◠", "◝", "◞", "◡", "◟"],
        "interval": 100,
    },
    "circle": {
        "frames": ["◡", "⊙", "◠"],
        "interval": 120,
    },
    "squareCorners": {
        "frames": ["◰", "◳", "◲", "◱"],
        "interval": 180,
    },
    "circleQuarters": {
        "frames": ["◴", "◷", "◶", "◵"],
        "interval": 120,
    },
    "circleHalves": {
        "frames": ["◐", "◓", "◑", "◒"],
        "interval": 50,
    },
    "squish": {
        "frames": ["╫", "╪"],
        "interval": 100,
    },
    "toggle": {
        "frames": ["⊶", "⊷"],
        "interval": 250,
    },
    "toggle2": {
        "frames": ["▫", "▪"],
        "interval": 80,
    },
    "toggle3": {
        "frames": ["□", "■"],
        "interval": 120,
    },
    "toggle4": {
        "frames": ["■", "□", "▪", "▫"],
        "interval": 100,
    },
    "toggle5": {
        "frames": ["▮", "▯"],
        "interval": 100,
    },
    "toggle6": {
        "frames": ["ဝ", "၀"],
        "interval": 300,
    },
    "toggle7": {
        "frames": ["⦾", "⦿"],
        "interval": 80,
    },
    "toggle8": {
        "frames": ["◍", "◌"],
        "interval": 100,
    },
    "toggle9": {
        "frames": ["◉", "◎"],
        "interval": 100,
    },
    "toggle10": {
        "frames": ["㊂", "㊀", "㊁"],
        "interval": 100,
    },
    "toggle11": {
        "frames": ["⧇", "⧆"],
        "interval": 50,
    },
    "toggle12": {
        "frames": ["☗", "☖"],
        "interval": 120,
    },
    "toggle13": {
        "frames": ["=", "*", "-"],
        "interval": 80,
    },
    "arrow": {
        "frames": ["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
        "interval": 100,
    },
    "arrow2": {
        "frames": ["⬆️ ", "↗️ ", "➡️ ", "↘️ ", "⬇️ ", "↙️ ", "⬅️ ", "↖️ "],
        "interval": 80,
    },
    "arrow3": {
        "frames": ["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"],
        "interval": 120,
    },
    "bouncingBar": {
        "frames": [
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]",
            "[  ==]", "[   =]", "[    ]", "[   =]", "[  ==]",
            "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
        ],
        "interval": 80,
    },
    "bouncingBall": {
        "frames": [
            "( ●    )", "(  ●   )", "(   ●  )", "(    ● )",
            "(     ●)", "(    ● )", "(   ●  )", "(  ●   )",
            "( ●    )", "(●     )",
        ],
        "interval": 80,
    },
    "clock": {
        "frames": ["🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚"],
        "interval": 100,
    },
    "earth": {
        "frames": ["🌍", "🌎", "🌏"],
        "interval": 180,
    },
    "moon": {
        "frames": ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
        "interval": 80,
    },
    "runner": {
        "frames": ["🚶", "🏃"],
        "interval": 140,
    },
    "pong": {
        "frames": [
            "▐⠂       ▌", "▐⠈       ▌", "▐ ⠂      ▌", "▐ ⠠      ▌",
            "▐  ⡀     ▌", "▐  ⠠     ▌", "▐   ⠂    ▌", "▐   ⠈    ▌",
            "▐    ⠂   ▌", "▐    ⠠   ▌", "▐     ⡀  ▌", "▐     ⠠  ▌",
            "▐      ⠂ ▌", "▐      ⠈ ▌", "▐       ⠂▌", "▐       ⠠▌",
            "▐       ⡀▌", "▐      ⠠ ▌", "▐      ⠂ ▌", "▐     ⠈  ▌",
            "▐     ⠂  ▌", "▐    ⠠   ▌", "▐    ⡀   ▌", "▐   ⠠    ▌",
            "▐   ⠂    ▌", "▐  ⠈     ▌", "▐  ⠂     ▌", "▐ ⠠      ▌",
            "▐ ⡀      ▌", "▐⠠       ▌",
        ],
        "interval": 80,
    },
    "shark": {
        "frames": [
            "▐|\\____________▌", "▐_|\\___________▌", "▐__|\\__________▌",
            "▐___|\\_________▌", "▐____|\\________▌", "▐_____|\\_______▌",
            "▐______|\\______▌", "▐_______|\\_____▌", "▐________|\\____▌",
            "▐_________|\\___▌", "▐__________|\\__▌", "▐___________|\\_▌",
            "▐____________|\\▌", "▐____________/|▌", "▐___________/|_▌",
            "▐__________/|__▌", "▐_________/|___▌", "▐________/|____▌",
            "▐_______/|_____▌", "▐______/|______▌", "▐_____/|_______▌",
            "▐____/|________▌", "▐___/|_________▌", "▐__/|__________▌",
            "▐_/|___________▌", "▐/|____________▌",
        ],
        "interval": 120,
    },
    "dqpb": {
        "frames": ["d", "q", "p", "b"],
        "interval": 100,
    },
    "weather": {
        "frames": ["☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "⛈ ", "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️", "🌤 ", "☀️ ", "☀️ "],
        "interval": 100,
    },
    "christmas": {
        "frames": ["🌲", "🎄"],
        "interval": 400,
    },
    "grenade": {
        "frames": ["،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁂", "   ", "   ", "   "],
        "interval": 80,
    },
    "point": {
        "frames": ["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
        "interval": 125,
    },
    "layer": {
        "frames": ["-", "=", "≡"],
        "interval": 150,
    },
}


class Spinner:
    """A spinner animation.
    
    Matches rich.spinner.Spinner API.
    """

    def __init__(
        self,
        name: str = "dots",
        text: Union[str, Text] = "",
        *,
        style: Optional[Union[str, Style]] = None,
        speed: float = 1.0,
    ) -> None:
        """Create a Spinner.
        
        Args:
            name: Spinner name from SPINNERS.
            text: Text to show after spinner.
            style: Spinner style.
            speed: Animation speed multiplier.
        """
        self.name = name
        self.text = text
        self.style = style
        self.speed = speed
        
        spinner_def = SPINNERS.get(name, SPINNERS["dots"])
        self.frames = spinner_def["frames"]
        self.interval = spinner_def["interval"] / speed
        
        self._frame_iter = itertools.cycle(self.frames)
        self._last_frame_time = 0.0
        self._current_frame = next(self._frame_iter)

    def __str__(self) -> str:
        """Get current frame."""
        return f"{self._current_frame} {self.text}"

    @property
    def frame(self) -> str:
        """Get the current frame character."""
        return self._current_frame

    def update(self) -> None:
        """Advance to the next frame if enough time has passed."""
        current_time = time.time() * 1000  # Convert to ms
        if current_time - self._last_frame_time >= self.interval:
            self._current_frame = next(self._frame_iter)
            self._last_frame_time = current_time

    def render(self, time_elapsed: float) -> Text:
        """Render the spinner at a given time.
        
        Args:
            time_elapsed: Time in seconds since start.
            
        Returns:
            Text with current frame.
        """
        frame_index = int(time_elapsed * 1000 / self.interval) % len(self.frames)
        frame = self.frames[frame_index]
        
        result = Text(f"{frame} ")
        if isinstance(self.text, Text):
            result.append(self.text.plain)
        else:
            result.append(str(self.text))
        
        return result

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        self.update()
        yield str(self)


__all__ = ["Spinner", "SPINNERS"]
