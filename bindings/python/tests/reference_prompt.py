from rich.prompt import Prompt, Confirm, IntPrompt
from rich.console import Console

console = Console()

console.rule("[bold red]Prompt Testing Reference")

# 1. Basic Prompt
name = Prompt.ask("Enter your [bold cyan]name[/]")
console.print(f"Hello, {name}!")

# 2. Prompt with Default
city = Prompt.ask("Enter your city", default="London")
console.print(f"City: {city}")

# 3. Prompt with Choices
size = Prompt.ask("Choose a size", choices=["small", "medium", "large"], default="medium")
console.print(f"Size: {size}")

# 4. Confirm
is_ready = Confirm.ask("Are you ready?", default=True)
console.print(f"Ready: {is_ready}")

# 5. IntPrompt
age = IntPrompt.ask("Enter your age", default=18)
console.print(f"Age: {age}")

console.rule("[bold red]End Reference")
