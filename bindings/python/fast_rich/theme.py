"""Theme support - matches rich.theme API."""

from __future__ import annotations

from typing import Dict, Optional, IO

from fast_rich.style import Style


class Theme:
    """A container for styles.
    
    Matches rich.theme.Theme API.
    """

    def __init__(
        self,
        styles: Optional[Dict[str, str]] = None,
        *,
        inherit: bool = True,
    ) -> None:
        """Create Theme.
        
        Args:
            styles: Dictionary of style names to style definitions.
            inherit: Inherit from default theme.
        """
        self.styles: Dict[str, Style] = {}
        self.inherit = inherit
        
        if styles:
            for name, style_def in styles.items():
                self.styles[name] = Style.parse(style_def) if isinstance(style_def, str) else style_def

    @classmethod
    def from_file(
        cls,
        config_file: IO[str],
        *,
        source: Optional[str] = None,
        inherit: bool = True,
    ) -> "Theme":
        """Load theme from file.
        
        Args:
            config_file: File object to read from.
            source: Source path (for error messages).
            inherit: Inherit from default theme.
            
        Returns:
            Theme instance.
        """
        styles = {}
        for line in config_file:
            line = line.strip()
            if line and not line.startswith("#"):
                if "=" in line:
                    name, style_def = line.split("=", 1)
                    styles[name.strip()] = style_def.strip()
        return cls(styles, inherit=inherit)

    @classmethod
    def read(
        cls,
        path: str,
        *,
        inherit: bool = True,
        encoding: str = "utf-8",
    ) -> "Theme":
        """Read theme from path.
        
        Args:
            path: Path to theme file.
            inherit: Inherit from default theme.
            encoding: File encoding.
            
        Returns:
            Theme instance.
        """
        with open(path, encoding=encoding) as f:
            return cls.from_file(f, source=path, inherit=inherit)

    def get_style(
        self,
        name: str,
        default: Optional[Style] = None,
    ) -> Optional[Style]:
        """Get a style by name.
        
        Args:
            name: Style name.
            default: Default style.
            
        Returns:
            Style or default.
        """
        return self.styles.get(name, default)

    def __getitem__(self, name: str) -> Style:
        """Get style by name."""
        return self.styles[name]

    def __contains__(self, name: str) -> bool:
        """Check if style exists."""
        return name in self.styles


class ThemeStack:
    """A stack of themes for inheritance."""

    def __init__(self, theme: Theme) -> None:
        self.themes = [theme]

    def push(self, theme: Theme) -> None:
        """Push theme onto stack."""
        self.themes.append(theme)

    def pop(self) -> Theme:
        """Pop theme from stack."""
        return self.themes.pop()

    def get_style(self, name: str) -> Optional[Style]:
        """Get style from top of stack."""
        for theme in reversed(self.themes):
            style = theme.get_style(name)
            if style is not None:
                return style
        return None


# Default theme
DEFAULT_STYLES = {
    "repr.number": "cyan",
    "repr.string": "green",
    "repr.bool": "bright_blue",
    "repr.none": "bright_magenta",
    "json.number": "cyan",
    "json.string": "green",
    "json.bool": "bright_blue",
    "json.null": "bright_magenta",
    "rule.line": "bright_green",
    "rule.text": "bright_green",
    "markdown.h1": "bold underline",
    "markdown.h2": "bold",
    "markdown.code": "cyan",
    "markdown.link": "blue underline",
    "progress.bar.complete": "bright_green",
    "progress.bar.finished": "bright_green",
    "progress.bar.pulse": "bright_yellow",
    "tree.line": "bright_blue",
    "status.spinner": "bright_green",
    "logging.level.debug": "bright_blue",
    "logging.level.info": "bright_green",
    "logging.level.warning": "bright_yellow",
    "logging.level.error": "bright_red",
    "logging.level.critical": "bold bright_red",
}

DEFAULT_THEME = Theme(DEFAULT_STYLES)


__all__ = ["Theme", "ThemeStack", "DEFAULT_THEME", "DEFAULT_STYLES"]
