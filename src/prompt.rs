//! Interactive prompt module.
//!
//! # EOF handling
//!
//! [`Prompt::try_ask`] returns [`Result`]; reading from a closed stdin yields
//! [`PromptError::Eof`] (or the configured default) instead of spinning
//! forever. [`Prompt::ask`] keeps the infallible signature and returns the
//! default on EOF, panicking only when no default exists.

use crate::console::Console;
use crate::screen::RawModeGuard;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;

/// Error returned by [`Prompt::try_ask`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptError {
    /// stdin reached end-of-file (e.g. piped input exhausted, Ctrl+D).
    Eof,
    /// The user aborted the prompt (Ctrl+C).
    Interrupted,
    /// An I/O error occurred while reading input.
    Io(String),
}

impl Display for PromptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptError::Eof => write!(f, "end of input while waiting for prompt"),
            PromptError::Interrupted => write!(f, "prompt interrupted"),
            PromptError::Io(e) => write!(f, "prompt I/O error: {e}"),
        }
    }
}

impl std::error::Error for PromptError {}

/// Interactive prompt to ask for user input.
#[derive(Debug, Clone)]
pub struct Prompt<T> {
    prompt: String,
    default: Option<T>,
    password: bool,
    choices: Option<Vec<T>>,
    show_default: bool,
    show_choices: bool,
    case_sensitive: bool,
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
            case_sensitive: true,
        }
    }

    /// Set a default value.
    pub fn default(mut self, default: T) -> Self {
        self.default = Some(default);
        self
    }

    /// Hide the input (for passwords).
    ///
    /// Input is read in raw mode and not echoed to the terminal.
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

    /// Whether choice matching is case-sensitive (default: `true`).
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Ask the question and return the result.
    ///
    /// On end-of-file this returns the default if one is set; otherwise it
    /// panics. Use [`Prompt::try_ask`] for fallible handling.
    ///
    /// # Panics
    ///
    /// Panics if stdin reaches EOF and no default is configured.
    pub fn ask(&self) -> T {
        self.try_ask()
            .unwrap_or_else(|e| panic!("prompt failed: {e}"))
    }

    /// Ask the question and return the result, fallibly.
    ///
    /// EOF on stdin yields [`PromptError::Eof`] (or the default, if set)
    /// instead of looping forever.
    pub fn try_ask(&self) -> Result<T, PromptError> {
        self.ask_with(|| read_input_line(self.password))
    }

    /// Core loop with an injectable reader (unit-testable).
    ///
    /// The reader returns `Ok(0)` on EOF, mirroring [`io::BufRead::read_line`].
    fn ask_with<F>(&self, mut read: F) -> Result<T, PromptError>
    where
        F: FnMut() -> io::Result<String>,
    {
        let console = Console::new();

        loop {
            self.render_prompt(&console);
            let _ = io::stdout().flush();

            let input = match read() {
                Ok(s) => s,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    return match self.default {
                        Some(ref def) => Ok(def.clone()),
                        None => Err(PromptError::Eof),
                    };
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    return Err(PromptError::Interrupted);
                }
                Err(e) => return Err(PromptError::Io(e.to_string())),
            };
            let trimmed = input.trim();

            if trimmed.is_empty() {
                // Blank line: use the default, or re-prompt. EOF cannot spin
                // here because the reader surfaces it as an error above.
                if let Some(ref def) = self.default {
                    return Ok(def.clone());
                }
                continue;
            }

            match T::from_str(trimmed) {
                Ok(val) => {
                    if let Some(ref choices) = self.choices {
                        if !self.choice_matches(choices, &val) {
                            console
                                .print("[bold red]Please select one of the available options[/]");
                            continue;
                        }
                    }
                    return Ok(val);
                }
                Err(_) => {
                    console.print("[bold red]Please enter a valid value[/]");
                }
            }
        }
    }

    /// Match a parsed value against the allowed choices, honoring
    /// `case_sensitive`.
    fn choice_matches(&self, choices: &[T], val: &T) -> bool {
        if self.case_sensitive {
            return choices.contains(val);
        }
        // Fall back to string comparison, case-insensitively.
        let v = val.to_string().to_lowercase();
        choices.iter().any(|c| c.to_string().to_lowercase() == v)
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

/// Read one line from stdin, or an error/EOF marker.
///
/// For password prompts this reads via crossterm raw mode so the input is
/// not echoed; the terminal is restored on every exit path (including
/// panics) via [`RawModeGuard`].
fn read_input_line(password: bool) -> io::Result<String> {
    if password {
        read_secret_line()
    } else {
        let mut input = String::new();
        let n = io::stdin().read_line(&mut input)?;
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "eof"));
        }
        Ok(input)
    }
}

/// Read a line without echo using raw mode + crossterm key events.
fn read_secret_line() -> io::Result<String> {
    let _guard = RawModeGuard::new()?;
    let mut out = io::stdout();
    let mut secret = String::new();

    loop {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    out.write_all(b"\n")?;
                    out.flush()?;
                    return Ok(secret);
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    out.write_all(b"\n")?;
                    out.flush()?;
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "interrupted"));
                }
                KeyCode::Backspace => {
                    if secret.pop().is_some() {
                        // Erase the last displayed position (no-op visually
                        // since we never echoed it, but keeps the cursor sane).
                        out.write_all(b"\x08 \x08")?;
                        out.flush()?;
                    }
                }
                KeyCode::Char(c) => {
                    secret.push(c);
                }
                _ => {}
            },
            _ => {}
        }
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
    ///
    /// On EOF the default (or `false` when none) is returned instead of
    /// hanging.
    pub fn ask(prompt: &str, default: Option<bool>) -> bool {
        let def_str = default
            .map(|v| if v { "y" } else { "n" })
            .unwrap_or("y")
            .to_string();

        let p = Prompt::<String>::new(prompt)
            .choices(&[
                "y".to_string(),
                "n".to_string(),
                "yes".to_string(),
                "no".to_string(),
            ])
            .case_sensitive(false)
            .default(def_str);

        let answer = p.try_ask().unwrap_or_else(|_| {
            // EOF / interrupted: fall back to the default answer.
            if default.unwrap_or(false) { "y" } else { "n" }.to_string()
        });
        answer.to_lowercase().starts_with('y')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn eof_reader() -> impl FnMut() -> io::Result<String> {
        || Err(io::Error::new(io::ErrorKind::UnexpectedEof, "eof"))
    }

    fn line_reader(lines: Vec<&'static str>) -> impl FnMut() -> io::Result<String> {
        let idx = Cell::new(0usize);
        move || {
            let i = idx.get();
            idx.set(i + 1);
            match lines.get(i) {
                Some(line) => Ok(line.to_string()),
                None => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "eof")),
            }
        }
    }

    #[test]
    fn test_eof_without_default_is_error() {
        let p = Prompt::<i64>::new("num");
        assert_eq!(p.ask_with(eof_reader()), Err(PromptError::Eof));
    }

    #[test]
    fn test_eof_with_default_returns_default() {
        let p = Prompt::<i64>::new("num").default(7);
        assert_eq!(p.ask_with(eof_reader()), Ok(7));
    }

    #[test]
    fn test_valid_input_parses() {
        let p = Prompt::<i64>::new("num");
        assert_eq!(p.ask_with(line_reader(vec!["42"])), Ok(42));
    }

    #[test]
    fn test_invalid_then_valid_retries_not_spins() {
        let p = Prompt::<i64>::new("num").default(-1);
        // Bad input re-prompts; then EOF returns the default.
        assert_eq!(p.ask_with(line_reader(vec!["abc"])), Ok(-1));
    }

    #[test]
    fn test_blank_line_uses_default() {
        let p = Prompt::<String>::new("name").default("anon".into());
        assert_eq!(p.ask_with(line_reader(vec![""])), Ok("anon".to_string()));
    }

    #[test]
    fn test_choices_reject_and_accept() {
        let p = Prompt::<String>::new("pick").choices(&["a".into(), "b".into()]);
        assert_eq!(p.ask_with(line_reader(vec!["c", "a"])), Ok("a".to_string()));
    }

    #[test]
    fn test_case_insensitive_choices() {
        let p = Prompt::<String>::new("pick")
            .choices(&["y".into(), "n".into()])
            .case_sensitive(false);
        assert_eq!(p.ask_with(line_reader(vec!["Y"])), Ok("Y".to_string()));
    }

    #[test]
    fn test_prompt_error_display() {
        assert!(PromptError::Eof.to_string().contains("end of input"));
    }
}
