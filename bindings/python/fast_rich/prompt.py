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
    """Raised by process_response when the answer is not valid."""

    def __init__(self, message: str) -> None:
        super().__init__(message)
        self.message = message


class PromptBase:
    """Base class for prompts.

    Matches rich.prompt.PromptBase: ``Prompt.ask(...)`` is a classmethod that
    builds an instance and calls it; calling the instance runs the input loop.
    """

    prompt_suffix: str = ": "
    illegal_choice_message: str = "[prompt.invalid.choice]Please select one of the available options"
    validate_error_message: str = "[prompt.invalid]Please enter a valid value"
    response_type: Type = str

    def __init__(
        self,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        password: bool = False,
        choices: Optional[List[str]] = None,
        case_sensitive: bool = True,
        show_default: bool = True,
        show_choices: bool = True,
    ) -> None:
        self.prompt = prompt
        self.console = console or Console()
        self.password = password
        if choices is not None:
            self.choices = choices
        elif not hasattr(self, "choices"):
            self.choices = None
        self.case_sensitive = case_sensitive
        self.show_default = show_default
        self.show_choices = show_choices

    @classmethod
    def ask(
        cls,
        prompt: str = "",
        *,
        console: Optional[Console] = None,
        password: bool = False,
        choices: Optional[List[str]] = None,
        case_sensitive: bool = True,
        show_default: bool = True,
        show_choices: bool = True,
        default: Any = ...,
        stream: Optional[Any] = None,
    ) -> Any:
        """Shortcut to construct a prompt and run it.

        Raises:
            EOFError: stdin was closed before an answer was given.
        """
        instance = cls(
            prompt,
            console=console,
            password=password,
            choices=choices,
            case_sensitive=case_sensitive,
            show_default=show_default,
            show_choices=show_choices,
        )
        return instance(default=default, stream=stream)

    def render_default(self, default: Any) -> str:
        """Render the default value as markup."""
        return f"[prompt.default]({default})"

    def make_prompt(self, default: Any) -> str:
        """Build the prompt markup string."""
        from fast_rich.markup import escape

        parts = [self.prompt]
        if self.choices and self.show_choices:
            parts.append(" [prompt.choices]" + escape("[" + "/".join(self.choices) + "]") + "[/]")
        if default is not ... and self.show_default and isinstance(default, (str, self.response_type)):
            parts.append(" " + self.render_default(default) + "[/]")
        parts.append(self.prompt_suffix)
        return "".join(parts)

    def check_choice(self, value: str) -> bool:
        """Check if value is a valid choice."""
        if self.choices is None:
            return True
        if self.case_sensitive:
            return value.strip() in self.choices
        return value.strip().lower() in [c.lower() for c in self.choices]

    def process_response(self, value: str) -> Any:
        """Convert and validate the response.

        Raises:
            InvalidResponse: the value is not valid.
        """
        value = value.strip()
        try:
            return_value = self.response_type(value)
        except ValueError:
            raise InvalidResponse(self.validate_error_message)
        if self.choices is not None:
            if not self.check_choice(value):
                raise InvalidResponse(self.illegal_choice_message)
            if not self.case_sensitive:
                return_value = self.response_type(
                    self.choices[[c.lower() for c in self.choices].index(value.lower())]
                )
        return return_value

    def on_validate_error(self, value: str, error: "InvalidResponse") -> None:
        """Print the validation error."""
        self.console.print(error.message)

    def get_input(self, prompt: str, password: bool, stream: Optional[Any] = None) -> str:
        """Read one line of input (raises EOFError when stdin is closed)."""
        return self.console.input(prompt, password=password, stream=stream)

    def pre_prompt(self) -> None:
        """Hook called before each prompt."""

    def __call__(self, *, default: Any = ..., stream: Optional[Any] = None) -> Any:
        """Run the prompt loop until a valid answer is given."""
        while True:
            self.pre_prompt()
            prompt = self.make_prompt(default)
            value = self.get_input(prompt, self.password, stream=stream)
            if value == "" and default is not ...:
                return default
            try:
                return self.process_response(value)
            except InvalidResponse as error:
                self.on_validate_error(value, error)
                continue


class Prompt(PromptBase):
    """A prompt for text input. Matches rich.prompt.Prompt."""

    response_type = str


class IntPrompt(PromptBase):
    """A prompt that returns an integer. Matches rich.prompt.IntPrompt."""

    response_type = int
    validate_error_message = "[prompt.invalid]Please enter a valid integer number"


class FloatPrompt(PromptBase):
    """A prompt that returns a float. Matches rich.prompt.FloatPrompt."""

    response_type = float
    validate_error_message = "[prompt.invalid]Please enter a number"


class Confirm(PromptBase):
    """A yes/no confirmation prompt. Matches rich.prompt.Confirm."""

    response_type = bool
    validate_error_message = "[prompt.invalid]Please enter Y or N"
    choices: Optional[List[str]] = ["y", "n"]

    def render_default(self, default: Any) -> str:
        yes, no = self.choices
        return f"[prompt.default]({yes if default else no})"

    def process_response(self, value: str) -> bool:
        value = value.strip().lower()
        if value not in self.choices:
            raise InvalidResponse(self.validate_error_message)
        return value == self.choices[0]


__all__ = [
    "Prompt",
    "Confirm",
    "IntPrompt",
    "FloatPrompt",
    "PromptBase",
    "PromptError",
    "InvalidResponse",
]
