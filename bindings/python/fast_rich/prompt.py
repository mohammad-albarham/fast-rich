"""Prompt utilities - matches rich.prompt API."""

from __future__ import annotations

from typing import Any, List, Optional, Type, TypeVar, Union

from fast_rich.console import Console
from fast_rich.text import Text


T = TypeVar("T")


class PromptError(Exception):
    """Exception for prompt errors."""
    pass


class InvalidResponse(PromptError):
    """Exception for invalid responses."""
    pass


class PromptBase:
    """Base class for prompts."""

    prompt_suffix: str = ": "
    illegal_choice_message: str = "Please select one of the available options"
    response_type: Type = str

    def __init__(
        self,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        password: bool = False,
        choices: Optional[List[str]] = None,
        show_default: bool = True,
        show_choices: bool = True,
    ) -> None:
        self.prompt = prompt
        self.console = console or Console()
        self.password = password
        self.choices = choices
        self.show_default = show_default
        self.show_choices = show_choices

    def render_default(self, default: Any) -> Text:
        """Render the default value."""
        return Text(f"({default})")

    def make_prompt(self, default: Any) -> str:
        """Build prompt string."""
        parts = [self.prompt]
        
        if self.choices and self.show_choices:
            choice_str = "/".join(self.choices)
            parts.append(f" [{choice_str}]")
        
        if default is not None and self.show_default:
            parts.append(f" ({default})")
        
        parts.append(self.prompt_suffix)
        return "".join(parts)

    def check_choice(self, value: str) -> bool:
        """Check if value is valid choice."""
        if self.choices is None:
            return True
        return value in self.choices

    def process_response(self, value: str) -> Any:
        """Process the response."""
        return self.response_type(value)

    def __call__(self, *, default: Any = None) -> Any:
        """Run the prompt."""
        return self.ask(default=default)

    def ask(self, *, default: Any = None) -> Any:
        """Ask for input."""
        prompt_str = self.make_prompt(default)
        
        while True:
            self.console.print(prompt_str, end="")
            
            if self.password:
                import getpass
                response = getpass.getpass("")
            else:
                response = input()
            
            if not response and default is not None:
                return default
            
            if not self.check_choice(response):
                self.console.print(self.illegal_choice_message)
                continue
            
            try:
                return self.process_response(response)
            except ValueError:
                self.console.print("Invalid input")
                continue


class Prompt(PromptBase):
    """A prompt for text input.
    
    Matches rich.prompt.Prompt API.
    """

    @classmethod
    def ask(
        cls,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        password: bool = False,
        choices: Optional[List[str]] = None,
        show_default: bool = True,
        show_choices: bool = True,
        default: Any = None,
        stream: Optional[Any] = None,
    ) -> str:
        """Ask for input.
        
        Args:
            prompt: Prompt text.
            console: Console to use.
            password: Hide input.
            choices: Valid choices.
            show_default: Show default value.
            show_choices: Show choices.
            default: Default value.
            stream: Input stream.
            
        Returns:
            User input.
        """
        instance = cls(
            prompt,
            console=console,
            password=password,
            choices=choices,
            show_default=show_default,
            show_choices=show_choices,
        )
        return instance(default=default)


class Confirm(PromptBase):
    """A yes/no confirmation prompt.
    
    Matches rich.prompt.Confirm API.
    """

    response_type = bool

    @classmethod
    def ask(
        cls,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        password: bool = False,
        show_default: bool = True,
        default: bool = False,
        stream: Optional[Any] = None,
    ) -> bool:
        """Ask for confirmation.
        
        Args:
            prompt: Prompt text.
            console: Console to use.
            password: Hide input.
            show_default: Show default.
            default: Default value.
            stream: Input stream.
            
        Returns:
            True for yes, False for no.
        """
        instance = cls(
            prompt,
            console=console,
            password=password,
            choices=["y", "n"],
            show_default=show_default,
            show_choices=True,
        )
        
        while True:
            result = instance(default="y" if default else "n")
            if result.lower() in ("y", "yes"):
                return True
            if result.lower() in ("n", "no"):
                return False


class IntPrompt(PromptBase):
    """Prompt for integer input."""

    response_type = int

    @classmethod
    def ask(
        cls,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        show_default: bool = True,
        default: Optional[int] = None,
        stream: Optional[Any] = None,
    ) -> int:
        """Ask for integer input."""
        instance = cls(
            prompt,
            console=console,
            show_default=show_default,
        )
        return instance(default=default)


class FloatPrompt(PromptBase):
    """Prompt for float input."""

    response_type = float

    @classmethod
    def ask(
        cls,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        show_default: bool = True,
        default: Optional[float] = None,
        stream: Optional[Any] = None,
    ) -> float:
        """Ask for float input."""
        instance = cls(
            prompt,
            console=console,
            show_default=show_default,
        )
        return instance(default=default)


__all__ = [
    "Prompt",
    "Confirm",
    "IntPrompt",
    "FloatPrompt",
    "PromptBase",
    "PromptError",
    "InvalidResponse",
]
