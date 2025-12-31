//! Box drawing characters and styles.
//!
//! This module defines the `Box` struct which holds the character set for drawing
//! boxes (panels, tables, etc.) and provides standard styles matching Python's `rich.box`.

/// A structure defining the characters used to draw a box.
///
/// A box is composed of a top, middle (for rows/sections), and bottom part.
/// Each part has a left, middle, right, and cross/divider character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Box {
    /// Top line
    pub top: Line,
    /// Header separator line
    pub head: Line,
    /// Middle line (row separator)
    pub mid: Line,
    /// Bottom line
    pub bottom: Line,
    /// Header row vertical lines
    pub header: Line,
    /// Content row vertical lines
    pub cell: Line,
}

/// A structure defining the characters for a single horizontal line in a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub left: char,
    pub mid: char,
    pub right: char,
    pub cross: char,
}

impl Line {
    pub const fn new(left: char, mid: char, right: char, cross: char) -> Self {
        Self {
            left,
            mid,
            right,
            cross,
        }
    }
}

/// Standard ASCII box style.
pub const ASCII: Box = Box {
    top: Line::new('+', '-', '+', '+'),
    head: Line::new('+', '-', '+', '+'),
    mid: Line::new('+', '-', '+', '+'),
    bottom: Line::new('+', '-', '+', '+'),
    header: Line::new('|', ' ', '|', '|'),
    cell: Line::new('|', ' ', '|', '|'),
};

/// ASCII box style with double-line header separator.
pub const ASCII_DOUBLE_HEAD: Box = Box {
    top: Line::new('+', '-', '+', '+'),
    head: Line::new('+', '=', '+', '+'),
    mid: Line::new('+', '-', '+', '+'),
    bottom: Line::new('+', '-', '+', '+'),
    header: Line::new('|', ' ', '|', '|'),
    cell: Line::new('|', ' ', '|', '|'),
};

/// Simple square box using standard drawing characters.
pub const SQUARE: Box = Box {
    top: Line::new('┌', '─', '┐', '┬'),
    head: Line::new('├', '─', '┤', '┼'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Square box with double-line header separator.
pub const SQUARE_DOUBLE_HEAD: Box = Box {
    top: Line::new('┌', '─', '┐', '┬'),
    head: Line::new('╞', '═', '╡', '╪'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Minimal box (open sides).
pub const MINIMAL: Box = Box {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '─', ' ', '─'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Minimal box with heavy header separator.
pub const MINIMAL_HEAVY_HEAD: Box = Box {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '━', ' ', '━'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Minimal box with double-line header separator.
pub const MINIMAL_DOUBLE_HEAD: Box = Box {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '═', ' ', '═'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Box with only horizontal lines.
pub const HORIZONTALS: Box = Box {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '─', ' ', '─'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Rounded corners (very popular).
pub const ROUNDED: Box = Box {
    top: Line::new('╭', '─', '╮', '┬'),
    head: Line::new('├', '─', '┤', '┼'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('╰', '─', '╯', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Heavy (bold) lines.
pub const HEAVY: Box = Box {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┣', '━', '┫', '╋'),
    mid: Line::new('┣', '━', '┫', '╋'),
    bottom: Line::new('┗', '━', '┛', '┻'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('┃', ' ', '┃', '┃'),
};

/// Heavy edges but light inner lines.
pub const HEAVY_EDGE: Box = Box {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┣', '━', '┫', '╇'),
    mid: Line::new('┣', '━', '┫', '╇'),
    bottom: Line::new('┗', '━', '┛', '┻'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('┃', ' ', '┃', '┃'),
};

/// Heavy header separator.
pub const HEAVY_HEAD: Box = Box {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┡', '━', '┩', '╇'),
    mid: Line::new('│', '─', '│', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Double lines.
pub const DOUBLE: Box = Box {
    top: Line::new('╔', '═', '╗', '╦'),
    head: Line::new('╠', '═', '╣', '╬'),
    mid: Line::new('╠', '═', '╣', '╬'),
    bottom: Line::new('╚', '═', '╝', '╩'),
    header: Line::new('║', ' ', '║', '║'),
    cell: Line::new('║', ' ', '║', '║'),
};

/// Double edges but single inner lines.
pub const DOUBLE_EDGE: Box = Box {
    top: Line::new('╔', '═', '╗', '╤'),
    head: Line::new('╠', '═', '╣', '╪'),
    mid: Line::new('╟', '─', '╢', '┼'),
    bottom: Line::new('╚', '═', '╝', '╧'),
    header: Line::new('║', ' ', '║', '║'),
    cell: Line::new('║', ' ', '║', '║'),
};
