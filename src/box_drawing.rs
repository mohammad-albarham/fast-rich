//! Box drawing characters and styles.
//!
//! This module defines the `Box` struct which holds the character set for drawing
//! boxes (panels, tables, etc.) and provides standard styles matching Python's `rich.box`.

/// A structure defining the characters used to draw a box.
///
/// A box is composed of a top, middle (for rows/sections), and bottom part.
/// Each part has a left, middle, right, and cross/divider character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxStyle {
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
pub const ASCII: BoxStyle = BoxStyle {
    top: Line::new('+', '-', '+', '+'),
    head: Line::new('+', '-', '+', '+'),
    mid: Line::new('+', '-', '+', '+'),
    bottom: Line::new('+', '-', '+', '+'),
    header: Line::new('|', ' ', '|', '|'),
    cell: Line::new('|', ' ', '|', '|'),
};

/// ASCII box style with double-line header separator.
pub const ASCII_DOUBLE_HEAD: BoxStyle = BoxStyle {
    top: Line::new('+', '-', '+', '+'),
    head: Line::new('+', '=', '+', '+'),
    mid: Line::new('+', '-', '+', '+'),
    bottom: Line::new('+', '-', '+', '+'),
    header: Line::new('|', ' ', '|', '|'),
    cell: Line::new('|', ' ', '|', '|'),
};

/// Simple square box using standard drawing characters.
pub const SQUARE: BoxStyle = BoxStyle {
    top: Line::new('┌', '─', '┐', '┬'),
    head: Line::new('├', '─', '┤', '┼'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Square box with double-line header separator.
pub const SQUARE_DOUBLE_HEAD: BoxStyle = BoxStyle {
    top: Line::new('┌', '─', '┐', '┬'),
    head: Line::new('╞', '═', '╡', '╪'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Minimal box (open sides).
pub const MINIMAL: BoxStyle = BoxStyle {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '─', ' ', '─'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Minimal box with heavy header separator.
pub const MINIMAL_HEAVY_HEAD: BoxStyle = BoxStyle {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '━', ' ', '━'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Minimal box with double-line header separator.
pub const MINIMAL_DOUBLE_HEAD: BoxStyle = BoxStyle {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '═', ' ', '═'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Box with only horizontal lines.
pub const HORIZONTALS: BoxStyle = BoxStyle {
    top: Line::new(' ', '─', ' ', '─'),
    head: Line::new(' ', '─', ' ', '─'),
    mid: Line::new(' ', '─', ' ', '─'),
    bottom: Line::new(' ', '─', ' ', '─'),
    header: Line::new(' ', ' ', ' ', ' '),
    cell: Line::new(' ', ' ', ' ', ' '),
};

/// Rounded corners (very popular).
pub const ROUNDED: BoxStyle = BoxStyle {
    top: Line::new('╭', '─', '╮', '┬'),
    head: Line::new('├', '─', '┤', '┼'),
    mid: Line::new('├', '─', '┤', '┼'),
    bottom: Line::new('╰', '─', '╯', '┴'),
    header: Line::new('│', ' ', '│', '│'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Heavy (bold) lines.
pub const HEAVY: BoxStyle = BoxStyle {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┣', '━', '┫', '╋'),
    mid: Line::new('┣', '━', '┫', '╋'),
    bottom: Line::new('┗', '━', '┛', '┻'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('┃', ' ', '┃', '┃'),
};

/// Heavy edges but light inner lines.
pub const HEAVY_EDGE: BoxStyle = BoxStyle {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┣', '━', '┫', '╇'),
    mid: Line::new('┣', '━', '┫', '╇'),
    bottom: Line::new('┗', '━', '┛', '┻'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('┃', ' ', '┃', '┃'),
};

/// Heavy header separator.
pub const HEAVY_HEAD: BoxStyle = BoxStyle {
    top: Line::new('┏', '━', '┓', '┳'),
    head: Line::new('┡', '━', '┩', '╇'),
    mid: Line::new('│', '─', '│', '┼'),
    bottom: Line::new('└', '─', '┘', '┴'),
    header: Line::new('┃', ' ', '┃', '┃'),
    cell: Line::new('│', ' ', '│', '│'),
};

/// Double lines.
pub const DOUBLE: BoxStyle = BoxStyle {
    top: Line::new('╔', '═', '╗', '╦'),
    head: Line::new('╠', '═', '╣', '╬'),
    mid: Line::new('╠', '═', '╣', '╬'),
    bottom: Line::new('╚', '═', '╝', '╩'),
    header: Line::new('║', ' ', '║', '║'),
    cell: Line::new('║', ' ', '║', '║'),
};

/// Double edges but single inner lines.
pub const DOUBLE_EDGE: BoxStyle = BoxStyle {
    top: Line::new('╔', '═', '╗', '╤'),
    head: Line::new('╠', '═', '╣', '╪'),
    mid: Line::new('╟', '─', '╢', '┼'),
    bottom: Line::new('╚', '═', '╝', '╧'),
    header: Line::new('║', ' ', '║', '║'),
    cell: Line::new('║', ' ', '║', '║'),
};

/// Old name for [`BoxStyle`] (shadows `std::Box` under glob imports).
#[deprecated(since = "0.3.3", note = "use `BoxStyle` instead")]
pub type Box = BoxStyle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_box_styles_have_complete_lines() {
        let styles = [
            ASCII,
            ASCII_DOUBLE_HEAD,
            SQUARE,
            SQUARE_DOUBLE_HEAD,
            MINIMAL,
            MINIMAL_HEAVY_HEAD,
            MINIMAL_DOUBLE_HEAD,
            HORIZONTALS,
            ROUNDED,
            HEAVY,
            HEAVY_EDGE,
            HEAVY_HEAD,
            DOUBLE,
            DOUBLE_EDGE,
        ];
        for style in styles {
            // Every line must define all six segments (even if blank).
            let _ = (
                style.top.left,
                style.head.cross,
                style.mid.right,
                style.bottom.mid,
                style.header.left,
                style.cell.cross,
            );
        }
    }

    #[test]
    fn test_line_new_builds_all_fields() {
        let line = Line::new('l', 'm', 'r', 'x');
        assert_eq!(line.left, 'l');
        assert_eq!(line.mid, 'm');
        assert_eq!(line.right, 'r');
        assert_eq!(line.cross, 'x');
    }
}
