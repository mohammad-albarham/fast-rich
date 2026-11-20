//! Terminal control codes (Rich's `rich.control` and `ControlType`):
//! ring the bell, move or hide the cursor, clear the screen, switch to the
//! alternate screen, set the window title.
//!
//! [`Control`] is a renderable: printing it writes its escape sequence and
//! nothing else. The escape strings are byte for byte the ones Rich 15
//! writes.
//!
//! ```
//! use fast_rich::control::Control;
//!
//! assert_eq!(Control::move_to(4, 2).to_string(), "\x1b[3;5H");
//! assert_eq!(Control::title("build").to_string(), "\x1b]0;build\x07");
//! ```

use std::fmt;

use crate::console::RenderContext;
use crate::renderable::{Renderable, Segment};
use crate::text::Span;

/// The kinds of control code (Rich's `ControlType`, same numbering).
///
/// ```
/// use fast_rich::control::ControlType;
/// assert_eq!(ControlType::SetWindowTitle as u8, 16);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ControlType {
    /// Ring the terminal bell.
    Bell = 1,
    /// Move the cursor to the start of the line.
    CarriageReturn = 2,
    /// Move the cursor to the top left corner.
    Home = 3,
    /// Clear the screen.
    Clear = 4,
    /// Show the cursor.
    ShowCursor = 5,
    /// Hide the cursor.
    HideCursor = 6,
    /// Switch to the alternate screen.
    EnableAltScreen = 7,
    /// Leave the alternate screen.
    DisableAltScreen = 8,
    /// Move the cursor up.
    CursorUp = 9,
    /// Move the cursor down.
    CursorDown = 10,
    /// Move the cursor right.
    CursorForward = 11,
    /// Move the cursor left.
    CursorBackward = 12,
    /// Move the cursor to a column.
    CursorMoveToColumn = 13,
    /// Move the cursor to a column and row.
    CursorMoveTo = 14,
    /// Erase part of the line.
    EraseInLine = 15,
    /// Set the terminal window title.
    SetWindowTitle = 16,
}

/// One control code with its parameters (Rich's `ControlCode` tuples).
///
/// ```
/// use fast_rich::control::{ControlCode, ControlType};
/// let code = ControlCode::CursorUp(3);
/// assert_eq!(code.control_type(), ControlType::CursorUp);
/// assert_eq!(code.escape(), "\x1b[3A");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlCode {
    /// `\x07`
    Bell,
    /// `\r`
    CarriageReturn,
    /// `ESC[H`
    Home,
    /// `ESC[2J`
    Clear,
    /// `ESC[?25h`
    ShowCursor,
    /// `ESC[?25l`
    HideCursor,
    /// `ESC[?1049h`
    EnableAltScreen,
    /// `ESC[?1049l`
    DisableAltScreen,
    /// `ESC[<n>A`
    CursorUp(usize),
    /// `ESC[<n>B`
    CursorDown(usize),
    /// `ESC[<n>C`
    CursorForward(usize),
    /// `ESC[<n>D`
    CursorBackward(usize),
    /// `ESC[<column + 1>G` (the column is 0-based).
    CursorMoveToColumn(usize),
    /// `ESC[<y + 1>;<x + 1>H` (0-based column `x` and row `y`).
    CursorMoveTo {
        /// Column (0-based).
        x: usize,
        /// Row (0-based).
        y: usize,
    },
    /// `ESC[<mode>K`: 0 erases to the end of the line, 1 to the start, 2
    /// the whole line.
    EraseInLine(usize),
    /// `ESC]0;<title>BEL`
    SetWindowTitle(String),
}

impl ControlCode {
    /// The kind of this code.
    ///
    /// ```
    /// use fast_rich::control::{ControlCode, ControlType};
    /// assert_eq!(ControlCode::Home.control_type(), ControlType::Home);
    /// ```
    pub fn control_type(&self) -> ControlType {
        match self {
            ControlCode::Bell => ControlType::Bell,
            ControlCode::CarriageReturn => ControlType::CarriageReturn,
            ControlCode::Home => ControlType::Home,
            ControlCode::Clear => ControlType::Clear,
            ControlCode::ShowCursor => ControlType::ShowCursor,
            ControlCode::HideCursor => ControlType::HideCursor,
            ControlCode::EnableAltScreen => ControlType::EnableAltScreen,
            ControlCode::DisableAltScreen => ControlType::DisableAltScreen,
            ControlCode::CursorUp(_) => ControlType::CursorUp,
            ControlCode::CursorDown(_) => ControlType::CursorDown,
            ControlCode::CursorForward(_) => ControlType::CursorForward,
            ControlCode::CursorBackward(_) => ControlType::CursorBackward,
            ControlCode::CursorMoveToColumn(_) => ControlType::CursorMoveToColumn,
            ControlCode::CursorMoveTo { .. } => ControlType::CursorMoveTo,
            ControlCode::EraseInLine(_) => ControlType::EraseInLine,
            ControlCode::SetWindowTitle(_) => ControlType::SetWindowTitle,
        }
    }

    /// The escape sequence for this code (Rich's `CONTROL_CODES_FORMAT`).
    ///
    /// ```
    /// use fast_rich::control::ControlCode;
    /// assert_eq!(ControlCode::CursorMoveToColumn(0).escape(), "\x1b[1G");
    /// ```
    pub fn escape(&self) -> String {
        match self {
            ControlCode::Bell => "\x07".into(),
            ControlCode::CarriageReturn => "\r".into(),
            ControlCode::Home => "\x1b[H".into(),
            ControlCode::Clear => "\x1b[2J".into(),
            ControlCode::ShowCursor => "\x1b[?25h".into(),
            ControlCode::HideCursor => "\x1b[?25l".into(),
            ControlCode::EnableAltScreen => "\x1b[?1049h".into(),
            ControlCode::DisableAltScreen => "\x1b[?1049l".into(),
            ControlCode::CursorUp(n) => format!("\x1b[{n}A"),
            ControlCode::CursorDown(n) => format!("\x1b[{n}B"),
            ControlCode::CursorForward(n) => format!("\x1b[{n}C"),
            ControlCode::CursorBackward(n) => format!("\x1b[{n}D"),
            ControlCode::CursorMoveToColumn(x) => format!("\x1b[{}G", x + 1),
            ControlCode::CursorMoveTo { x, y } => format!("\x1b[{};{}H", y + 1, x + 1),
            ControlCode::EraseInLine(mode) => format!("\x1b[{mode}K"),
            ControlCode::SetWindowTitle(title) => format!("\x1b]0;{title}\x07"),
        }
    }
}

/// A renderable that writes control codes: non-printing escapes that may
/// move the cursor (Rich's `Control`).
///
/// Rendering gives one control [`Span`] (see
/// [`Span::is_control`](crate::text::Span::is_control)), or nothing when
/// there are no codes. It takes no cells.
///
/// ```
/// use fast_rich::console::Console;
/// use fast_rich::control::Control;
///
/// let console = Console::capture();
/// console.print_renderable(&Control::show_cursor(false));
/// assert_eq!(console.get_captured_output(), "\x1b[?25l");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Control {
    codes: Vec<ControlCode>,
    text: String,
}

impl Control {
    /// A control from any codes, written in order.
    ///
    /// ```
    /// use fast_rich::control::{Control, ControlCode};
    /// let c = Control::new([ControlCode::Home, ControlCode::Clear]);
    /// assert_eq!(c.to_string(), "\x1b[H\x1b[2J");
    /// ```
    pub fn new(codes: impl IntoIterator<Item = ControlCode>) -> Self {
        let codes: Vec<ControlCode> = codes.into_iter().collect();
        let text = codes.iter().map(ControlCode::escape).collect();
        Control { codes, text }
    }

    /// The codes, in order.
    ///
    /// ```
    /// use fast_rich::control::{Control, ControlCode};
    /// assert_eq!(Control::bell().codes(), [ControlCode::Bell]);
    /// ```
    pub fn codes(&self) -> &[ControlCode] {
        &self.codes
    }

    /// Ring the bell.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::bell().to_string(), "\x07");
    /// ```
    pub fn bell() -> Self {
        Self::new([ControlCode::Bell])
    }

    /// Move the cursor to the top left corner.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::home().to_string(), "\x1b[H");
    /// ```
    pub fn home() -> Self {
        Self::new([ControlCode::Home])
    }

    /// Move the cursor relative to where it is: `x` columns right (left if
    /// negative), then `y` rows down (up if negative). Rich's
    /// `Control.move` (`move` is a Rust keyword).
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::move_cursor(2, -1).to_string(), "\x1b[2C\x1b[1A");
    /// assert_eq!(Control::move_cursor(0, 0).to_string(), "");
    /// ```
    pub fn move_cursor(x: i64, y: i64) -> Self {
        let mut codes = Vec::new();
        if x != 0 {
            let n = x.unsigned_abs() as usize;
            codes.push(if x > 0 {
                ControlCode::CursorForward(n)
            } else {
                ControlCode::CursorBackward(n)
            });
        }
        if y != 0 {
            let n = y.unsigned_abs() as usize;
            codes.push(if y > 0 {
                ControlCode::CursorDown(n)
            } else {
                ControlCode::CursorUp(n)
            });
        }
        Self::new(codes)
    }

    /// Move to column `x` (0-based), then `y` rows down (up if negative).
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::move_to_column(3, 0).to_string(), "\x1b[4G");
    /// assert_eq!(Control::move_to_column(0, -2).to_string(), "\x1b[1G\x1b[2A");
    /// ```
    pub fn move_to_column(x: usize, y: i64) -> Self {
        let mut codes = vec![ControlCode::CursorMoveToColumn(x)];
        if y != 0 {
            let n = y.unsigned_abs() as usize;
            codes.push(if y > 0 {
                ControlCode::CursorDown(n)
            } else {
                ControlCode::CursorUp(n)
            });
        }
        Self::new(codes)
    }

    /// Move the cursor to column `x`, row `y` (both 0-based).
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::move_to(0, 0).to_string(), "\x1b[1;1H");
    /// ```
    pub fn move_to(x: usize, y: usize) -> Self {
        Self::new([ControlCode::CursorMoveTo { x, y }])
    }

    /// Clear the screen.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::clear().to_string(), "\x1b[2J");
    /// ```
    pub fn clear() -> Self {
        Self::new([ControlCode::Clear])
    }

    /// Show or hide the cursor.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::show_cursor(true).to_string(), "\x1b[?25h");
    /// ```
    pub fn show_cursor(show: bool) -> Self {
        Self::new([if show {
            ControlCode::ShowCursor
        } else {
            ControlCode::HideCursor
        }])
    }

    /// Enter (and move home) or leave the alternate screen.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::alt_screen(true).to_string(), "\x1b[?1049h\x1b[H");
    /// assert_eq!(Control::alt_screen(false).to_string(), "\x1b[?1049l");
    /// ```
    pub fn alt_screen(enable: bool) -> Self {
        if enable {
            Self::new([ControlCode::EnableAltScreen, ControlCode::Home])
        } else {
            Self::new([ControlCode::DisableAltScreen])
        }
    }

    /// Set the terminal window title.
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert_eq!(Control::title("hi").to_string(), "\x1b]0;hi\x07");
    /// ```
    pub fn title(title: &str) -> Self {
        Self::new([ControlCode::SetWindowTitle(title.to_string())])
    }

    /// The control as a span (Rich's `Control.segment`).
    ///
    /// ```
    /// use fast_rich::control::Control;
    /// assert!(Control::clear().span().is_control());
    /// ```
    pub fn span(&self) -> Span {
        Span::raw(self.text.clone())
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

impl Renderable for Control {
    fn render(&self, _context: &RenderContext) -> Vec<Segment> {
        if self.text.is_empty() {
            Vec::new()
        } else {
            vec![Segment::from_span(self.span())]
        }
    }

    fn min_width(&self) -> usize {
        0
    }

    fn max_width(&self) -> usize {
        0
    }
}

/// Remove the control characters bell, backspace, vertical tab, form feed
/// and carriage return (Rich's `strip_control_codes`).
///
/// ```
/// use fast_rich::control::strip_control_codes;
/// assert_eq!(strip_control_codes("a\x07b\rc"), "abc");
/// ```
pub fn strip_control_codes(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '\x07' | '\x08' | '\x0b' | '\x0c' | '\r'))
        .collect()
}

/// Replace the same control characters with their escaped form (`\a`,
/// `\b`, `\v`, `\f`, `\r`; Rich's `escape_control_codes`).
///
/// ```
/// use fast_rich::control::escape_control_codes;
/// assert_eq!(escape_control_codes("a\rb"), "a\\rb");
/// ```
pub fn escape_control_codes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\x0b' => out.push_str("\\v"),
            '\x0c' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            _ => out.push(c),
        }
    }
    out
}
