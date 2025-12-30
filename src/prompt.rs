//! Interactive prompt module.

use crate::console::Console;

use std::io::{self, Write};

/// Interactive prompt to ask for user input.
pub struct Prompt;

impl Prompt {
    /// Ask a question to the user and return their input.
    pub fn ask(prompt: &str, default: Option<&str>, password: bool) -> String {
        let console = Console::new();

        // Format the prompt
        let prompt_text = if let Some(def) = default {
            format!("{} [bold]([/][bold cyan]{}[/][bold])[/]: ", prompt, def)
        } else {
            format!("{}: ", prompt)
        };

        console.print(&prompt_text);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if password {
            // In a real TUI we'd use crossterm to hide input, but for MVP standard stdin is used.
            // Password masking requires terminal manipulation not currently in minimal scope,
            // but we will support reading the line.
            // TODO: Add proper password masking.
            io::stdin().read_line(&mut input).unwrap_or_default();
        } else {
            io::stdin().read_line(&mut input).unwrap_or_default();
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            if let Some(def) = default {
                return def.to_string();
            }
        }

        trimmed.to_string()
    }
}
