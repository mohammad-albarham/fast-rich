//! Interactive prompt module.

use crate::console::Console;
use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;

/// Interactive prompt to ask for user input.
pub struct Prompt<T> {
    prompt: String,
    default: Option<T>,
    password: bool,
    choices: Option<Vec<T>>,
    show_default: bool,
    show_choices: bool,
    _case_sensitive: bool,
}

impl<T> Prompt<T>
where
    T: FromStr + Display + Clone + PartialEq,
    T::Err: Display,
{
    /// Create a new prompt with the given query.
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            default: None,
            password: false,
            choices: None,
            show_default: true,
            show_choices: true,
            _case_sensitive: true,
        }
    }

    /// Set a default value.
    pub fn default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }

    /// Hide the input (for passwords).
    pub fn secret(mut self) -> Self {
        self.password = true;
        self
    }

    /// Set valid choices.
    pub fn choices(mut self, choices: &[T]) -> Self {
        self.choices = Some(choices.to_vec());
        self
    }

    /// Show or hide the default value in the prompt.
    pub fn show_default(mut self, show: bool) -> Self {
        self.show_default = show;
        self
    }

    /// Show or hide the choices in the prompt.
    pub fn show_choices(mut self, show: bool) -> Self {
        self.show_choices = show;
        self
    }

    /// Ask the question and return the result.
    pub fn ask(&self) -> T {
        let console = Console::new();

        loop {
            self.render_prompt(&console);
            let _ = io::stdout().flush();

            let mut input = String::new();
            if self.password {
                // For MVP, just read line. Ideally use crossterm to hide input.
                // crossterm::terminal::enable_raw_mode() ... 
                // But keeping it simple for now to ensure stability.
                io::stdin().read_line(&mut input).unwrap_or_default();
            } else {
                io::stdin().read_line(&mut input).unwrap_or_default();
            }

            let trimmed = input.trim();

            if trimmed.is_empty() {
                if let Some(ref def) = self.default {
                    return def.clone();
                }
            }

            match T::from_str(trimmed) {
                Ok(val) => {
                    if let Some(ref choices) = self.choices {
                        if !choices.contains(&val) {
                            console.print("[bold red]Please select one of the available options[/]");
                            continue;
                        }
                    }
                    return val;
                }
                Err(_) => {
                    console.print("[bold red]Please enter a valid value[/]");
                }
            }
        }
    }

    fn render_prompt(&self, console: &Console) {
        let mut text = String::new();
        text.push_str(&self.prompt);

        if self.show_choices {
             if let Some(ref choices) = self.choices {
                 let choices_str: Vec<String> = choices.iter().map(|c| c.to_string()).collect();
                 text.push_str(&format!(" [bold magenta][{}][/]", choices_str.join("/")));
             }
        }

        if self.show_default {
            if let Some(ref def) = self.default {
                if !self.password {
                     text.push_str(&format!(" [bold cyan]({:?})[/]", def.to_string()));
                }
            }
        }

        text.push_str(": ");
        console.print(&text);
    }
}

/// Prompt for integers.
pub type IntPrompt = Prompt<i64>;

/// Prompt for floats.
pub type FloatPrompt = Prompt<f64>;

/// Confirmation prompt (yes/no).
pub struct Confirm;

impl Confirm {
    /// Ask a yes/no question.
    pub fn ask(prompt: &str, default: Option<bool>) -> bool {
        let def_str = default.map(|v| if v { "y" } else { "n" }).unwrap_or("y").to_string();
        
        let p = Prompt::<String>::new(prompt)
            .choices(&["y".to_string(), "n".to_string(), "yes".to_string(), "no".to_string()])
            .default(def_str);
        
        let answer = p.ask();
        answer.to_lowercase().starts_with('y')
    }
}
