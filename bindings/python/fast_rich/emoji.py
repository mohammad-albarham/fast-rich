"""Emoji support - matches rich.emoji API."""

from __future__ import annotations

from typing import Optional

# Common emoji mappings (subset of full emoji database)
EMOJI_MAP = {
    # Smileys
    ":smile:": "😄",
    ":grinning:": "😀",
    ":joy:": "😂",
    ":heart_eyes:": "😍",
    ":wink:": "😉",
    ":thinking:": "🤔",
    ":sunglasses:": "😎",
    ":sob:": "😭",
    ":angry:": "😠",
    ":scream:": "😱",
    
    # Gestures
    ":thumbsup:": "👍",
    ":thumbs_up:": "👍",
    ":+1:": "👍",
    ":thumbsdown:": "👎",
    ":thumbs_down:": "👎",
    ":-1:": "👎",
    ":clap:": "👏",
    ":wave:": "👋",
    ":raised_hands:": "🙌",
    ":pray:": "🙏",
    ":muscle:": "💪",
    ":point_right:": "👉",
    ":point_left:": "👈",
    ":point_up:": "👆",
    ":point_down:": "👇",
    ":ok_hand:": "👌",
    
    # Hearts
    ":heart:": "❤️",
    ":red_heart:": "❤️",
    ":orange_heart:": "🧡",
    ":yellow_heart:": "💛",
    ":green_heart:": "💚",
    ":blue_heart:": "💙",
    ":purple_heart:": "💜",
    ":broken_heart:": "💔",
    ":sparkling_heart:": "💖",
    
    # Objects
    ":rocket:": "🚀",
    ":fire:": "🔥",
    ":star:": "⭐",
    ":star2:": "🌟",
    ":sparkles:": "✨",
    ":zap:": "⚡",
    ":boom:": "💥",
    ":bulb:": "💡",
    ":warning:": "⚠️",
    ":x:": "❌",
    ":white_check_mark:": "✅",
    ":heavy_check_mark:": "✔️",
    ":ballot_box_with_check:": "☑️",
    ":question:": "❓",
    ":exclamation:": "❗",
    
    # Animals
    ":dog:": "🐕",
    ":cat:": "🐈",
    ":snake:": "🐍",
    ":bug:": "🐛",
    ":bee:": "🐝",
    ":butterfly:": "🦋",
    ":turtle:": "🐢",
    ":crab:": "🦀",
    ":unicorn:": "🦄",
    ":dragon:": "🐉",
    
    # Food
    ":pizza:": "🍕",
    ":hamburger:": "🍔",
    ":coffee:": "☕",
    ":beer:": "🍺",
    ":wine_glass:": "🍷",
    ":cake:": "🍰",
    ":apple:": "🍎",
    ":banana:": "🍌",
    
    # Tech
    ":computer:": "💻",
    ":keyboard:": "⌨️",
    ":iphone:": "📱",
    ":cd:": "💿",
    ":floppy_disk:": "💾",
    ":gear:": "⚙️",
    ":wrench:": "🔧",
    ":hammer:": "🔨",
    ":lock:": "🔒",
    ":key:": "🔑",
    
    # Weather
    ":sun:": "☀️",
    ":cloud:": "☁️",
    ":rain:": "🌧️",
    ":rainbow:": "🌈",
    ":snowflake:": "❄️",
    ":umbrella:": "☂️",
    
    # Arrows
    ":arrow_right:": "➡️",
    ":arrow_left:": "⬅️",
    ":arrow_up:": "⬆️",
    ":arrow_down:": "⬇️",
    
    # Misc
    ":party_popper:": "🎉",
    ":tada:": "🎉",
    ":gift:": "🎁",
    ":trophy:": "🏆",
    ":medal:": "🏅",
    ":crown:": "👑",
    ":gem:": "💎",
    ":money_bag:": "💰",
    ":chart_with_upwards_trend:": "📈",
    ":chart_with_downwards_trend:": "📉",
    ":clock:": "🕐",
    ":hourglass:": "⌛",
    ":bell:": "🔔",
    ":loudspeaker:": "📢",
    ":bookmark:": "🔖",
    ":link:": "🔗",
    ":paperclip:": "📎",
}


class Emoji:
    """A single emoji by name.
    
    Matches rich.emoji.Emoji API.
    """

    def __init__(
        self,
        name: str,
        *,
        style: Optional[str] = None,
    ) -> None:
        """Create an Emoji.
        
        Args:
            name: Emoji name (with or without colons).
            style: Optional style.
        """
        # Normalize name
        if not name.startswith(":"):
            name = f":{name}:"
        if not name.endswith(":"):
            name = f"{name}:"
            
        self.name = name
        self.style = style

    @property
    def emoji(self) -> str:
        """Get the emoji character."""
        return EMOJI_MAP.get(self.name, self.name)

    def __str__(self) -> str:
        """Return the emoji character."""
        return self.emoji

    def __repr__(self) -> str:
        """Return repr."""
        return f"Emoji({self.name!r})"

    def __rich_console__(self, console, options):
        """Rich console protocol."""
        yield self.emoji


def replace(text: str) -> str:
    """Replace emoji shortcodes with emoji characters.
    
    Args:
        text: Text with emoji shortcodes like :smile:
        
    Returns:
        Text with shortcodes replaced by emoji.
    """
    result = text
    for shortcode, emoji in EMOJI_MAP.items():
        result = result.replace(shortcode, emoji)
    return result


__all__ = ["Emoji", "EMOJI_MAP", "replace"]
