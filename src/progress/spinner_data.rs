//! Spinner frame definitions.
//!
//! Spinners are sourced from cli-spinners:
//!
//! MIT License
//! Copyright (c) Sindre Sorhus <sindresorhus@gmail.com> (sindresorhus.com)
//! https://github.com/sindresorhus/cli-spinners
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights to
//! use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software is furnished to do so,
//! subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included
//! in all copies or substantial portions of the Software.

/// Spinner definition with frames and interval.
#[derive(Debug, Clone)]
pub struct SpinnerDef {
    /// Animation frames
    pub frames: &'static [&'static str],
    /// Interval between frames in milliseconds
    pub interval_ms: u64,
}

// ============================================================================
// BRAILLE DOTS SPINNERS
// ============================================================================

pub const DOTS: SpinnerDef = SpinnerDef {
    frames: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
    interval_ms: 80,
};

pub const DOTS2: SpinnerDef = SpinnerDef {
    frames: &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
    interval_ms: 80,
};

pub const DOTS3: SpinnerDef = SpinnerDef {
    frames: &["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
    interval_ms: 80,
};

pub const DOTS4: SpinnerDef = SpinnerDef {
    frames: &[
        "⠄", "⠆", "⠇", "⠋", "⠙", "⠸", "⠰", "⠠", "⠰", "⠸", "⠙", "⠋", "⠇", "⠆",
    ],
    interval_ms: 80,
};

pub const DOTS5: SpinnerDef = SpinnerDef {
    frames: &[
        "⠋", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠦", "⠖", "⠒", "⠐", "⠐", "⠒", "⠓", "⠋",
    ],
    interval_ms: 80,
};

pub const DOTS6: SpinnerDef = SpinnerDef {
    frames: &[
        "⠁", "⠉", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠤", "⠄", "⠄", "⠤", "⠴", "⠲", "⠒", "⠂",
        "⠂", "⠒", "⠚", "⠙", "⠉", "⠁",
    ],
    interval_ms: 80,
};

pub const DOTS7: SpinnerDef = SpinnerDef {
    frames: &[
        "⠈", "⠉", "⠋", "⠓", "⠒", "⠐", "⠐", "⠒", "⠖", "⠦", "⠤", "⠠", "⠠", "⠤", "⠦", "⠖", "⠒", "⠐",
        "⠐", "⠒", "⠓", "⠋", "⠉", "⠈",
    ],
    interval_ms: 80,
};

pub const DOTS8: SpinnerDef = SpinnerDef {
    frames: &[
        "⠁", "⠁", "⠉", "⠙", "⠚", "⠒", "⠂", "⠂", "⠒", "⠲", "⠴", "⠤", "⠄", "⠄", "⠤", "⠠", "⠠", "⠤",
        "⠦", "⠖", "⠒", "⠐", "⠐", "⠒", "⠓", "⠋", "⠉", "⠈", "⠈",
    ],
    interval_ms: 80,
};

pub const DOTS9: SpinnerDef = SpinnerDef {
    frames: &["⢹", "⢺", "⢼", "⣸", "⣇", "⡧", "⡗", "⡏"],
    interval_ms: 80,
};

pub const DOTS10: SpinnerDef = SpinnerDef {
    frames: &["⢄", "⢂", "⢁", "⡁", "⡈", "⡐", "⡠"],
    interval_ms: 80,
};

pub const DOTS11: SpinnerDef = SpinnerDef {
    frames: &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
    interval_ms: 100,
};

pub const DOTS12: SpinnerDef = SpinnerDef {
    frames: &[
        "⢀⠀", "⡀⠀", "⠄⠀", "⢂⠀", "⡂⠀", "⠅⠀", "⢃⠀", "⡃⠀", "⠍⠀", "⢋⠀", "⡋⠀", "⠍⠁", "⢋⠁", "⡋⠁", "⠍⠉",
        "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⢈⠩", "⡀⢙", "⠄⡙", "⢂⠩", "⡂⢘", "⠅⡘", "⢃⠨", "⡃⢐",
        "⠍⡐", "⢋⠠", "⡋⢀", "⠍⡁", "⢋⠁", "⡋⠁", "⠍⠉", "⠋⠉", "⠋⠉", "⠉⠙", "⠉⠙", "⠉⠩", "⠈⢙", "⠈⡙", "⠈⠩",
        "⠀⢙", "⠀⡙", "⠀⠩", "⠀⢘", "⠀⡘", "⠀⠨", "⠀⢐", "⠀⡐", "⠀⠠", "⠀⢀", "⠀⡀",
    ],
    interval_ms: 80,
};

pub const DOTS13: SpinnerDef = SpinnerDef {
    frames: &["⣼", "⣹", "⢻", "⠿", "⡟", "⣏", "⣧", "⣶"],
    interval_ms: 80,
};

pub const DOTS14: SpinnerDef = SpinnerDef {
    frames: &[
        "⠉⠉", "⠈⠙", "⠀⠹", "⠀⢸", "⠀⣰", "⢀⣠", "⣀⣀", "⣄⡀", "⣆⠀", "⡇⠀", "⠏⠀", "⠋⠁",
    ],
    interval_ms: 80,
};

pub const DOTS_CIRCLE: SpinnerDef = SpinnerDef {
    frames: &["⢎ ", "⠎⠁", "⠊⠑", "⠈⠱", " ⡱", "⢀⡰", "⢄⡠", "⢆⡀"],
    interval_ms: 80,
};

pub const SAND: SpinnerDef = SpinnerDef {
    frames: &[
        "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮", "⣶",
        "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡", "⢁",
    ],
    interval_ms: 80,
};

pub const BOUNCE: SpinnerDef = SpinnerDef {
    frames: &["⠁", "⠂", "⠄", "⠂"],
    interval_ms: 120,
};

// ============================================================================
// LINE SPINNERS
// ============================================================================

pub const LINE: SpinnerDef = SpinnerDef {
    frames: &["-", "\\", "|", "/"],
    interval_ms: 130,
};

pub const LINE2: SpinnerDef = SpinnerDef {
    frames: &["⠂", "-", "–", "—", "–", "-"],
    interval_ms: 100,
};

pub const PIPE: SpinnerDef = SpinnerDef {
    frames: &["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
    interval_ms: 100,
};

pub const ROLLING_LINE: SpinnerDef = SpinnerDef {
    frames: &["/  ", " - ", " \\ ", "  |", "  |", " \\ ", " - ", "/  "],
    interval_ms: 80,
};

pub const SIMPLE_DOTS: SpinnerDef = SpinnerDef {
    frames: &[".  ", ".. ", "...", "   "],
    interval_ms: 400,
};

pub const SIMPLE_DOTS_SCROLLING: SpinnerDef = SpinnerDef {
    frames: &[".  ", ".. ", "...", " ..", "  .", "   "],
    interval_ms: 200,
};

// ============================================================================
// STAR SPINNERS
// ============================================================================

pub const STAR: SpinnerDef = SpinnerDef {
    frames: &["✶", "✸", "✹", "✺", "✹", "✷"],
    interval_ms: 70,
};

pub const STAR2: SpinnerDef = SpinnerDef {
    frames: &["+", "x", "*"],
    interval_ms: 80,
};

// ============================================================================
// SHAPE SPINNERS
// ============================================================================

pub const ARC: SpinnerDef = SpinnerDef {
    frames: &["◜", "◠", "◝", "◞", "◡", "◟"],
    interval_ms: 100,
};

pub const CIRCLE: SpinnerDef = SpinnerDef {
    frames: &["◡", "⊙", "◠"],
    interval_ms: 120,
};

pub const CIRCLE_HALVES: SpinnerDef = SpinnerDef {
    frames: &["◐", "◓", "◑", "◒"],
    interval_ms: 50,
};

pub const CIRCLE_QUARTERS: SpinnerDef = SpinnerDef {
    frames: &["◴", "◷", "◶", "◵"],
    interval_ms: 120,
};

pub const SQUARE_CORNERS: SpinnerDef = SpinnerDef {
    frames: &["◰", "◳", "◲", "◱"],
    interval_ms: 180,
};

pub const TRIANGLE: SpinnerDef = SpinnerDef {
    frames: &["◢", "◣", "◤", "◥"],
    interval_ms: 50,
};

pub const BINARY: SpinnerDef = SpinnerDef {
    frames: &[
        "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000", "110011",
        "110101",
    ],
    interval_ms: 80,
};

pub const SQUISH: SpinnerDef = SpinnerDef {
    frames: &["╫", "╪"],
    interval_ms: 100,
};

pub const FLIP: SpinnerDef = SpinnerDef {
    frames: &["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
    interval_ms: 70,
};

pub const HAMBURGER: SpinnerDef = SpinnerDef {
    frames: &["☱", "☲", "☴"],
    interval_ms: 100,
};

// ============================================================================
// BOX SPINNERS
// ============================================================================

pub const BOX_BOUNCE: SpinnerDef = SpinnerDef {
    frames: &["▖", "▘", "▝", "▗"],
    interval_ms: 120,
};

pub const BOX_BOUNCE2: SpinnerDef = SpinnerDef {
    frames: &["▌", "▀", "▐", "▄"],
    interval_ms: 100,
};

pub const NOISE: SpinnerDef = SpinnerDef {
    frames: &["▓", "▒", "░"],
    interval_ms: 100,
};

// ============================================================================
// GROWING SPINNERS
// ============================================================================

pub const GROW_VERTICAL: SpinnerDef = SpinnerDef {
    frames: &["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃"],
    interval_ms: 120,
};

pub const GROW_HORIZONTAL: SpinnerDef = SpinnerDef {
    frames: &["▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"],
    interval_ms: 120,
};

pub const BALLOON: SpinnerDef = SpinnerDef {
    frames: &[" ", ".", "o", "O", "@", "*", " "],
    interval_ms: 140,
};

pub const BALLOON2: SpinnerDef = SpinnerDef {
    frames: &[".", "o", "O", "°", "O", "o", "."],
    interval_ms: 120,
};

// ============================================================================
// TOGGLE SPINNERS
// ============================================================================

pub const TOGGLE: SpinnerDef = SpinnerDef {
    frames: &["⊶", "⊷"],
    interval_ms: 250,
};

pub const TOGGLE2: SpinnerDef = SpinnerDef {
    frames: &["▫", "▪"],
    interval_ms: 80,
};

pub const TOGGLE3: SpinnerDef = SpinnerDef {
    frames: &["□", "■"],
    interval_ms: 120,
};

pub const TOGGLE4: SpinnerDef = SpinnerDef {
    frames: &["■", "□", "▪", "▫"],
    interval_ms: 100,
};

pub const TOGGLE5: SpinnerDef = SpinnerDef {
    frames: &["▮", "▯"],
    interval_ms: 100,
};

pub const TOGGLE6: SpinnerDef = SpinnerDef {
    frames: &["ဝ", "၀"],
    interval_ms: 300,
};

pub const TOGGLE7: SpinnerDef = SpinnerDef {
    frames: &["⦾", "⦿"],
    interval_ms: 80,
};

pub const TOGGLE8: SpinnerDef = SpinnerDef {
    frames: &["◍", "◌"],
    interval_ms: 100,
};

pub const TOGGLE9: SpinnerDef = SpinnerDef {
    frames: &["◉", "◎"],
    interval_ms: 100,
};

pub const TOGGLE10: SpinnerDef = SpinnerDef {
    frames: &["㊂", "㊀", "㊁"],
    interval_ms: 100,
};

pub const TOGGLE11: SpinnerDef = SpinnerDef {
    frames: &["⧇", "⧆"],
    interval_ms: 50,
};

pub const TOGGLE12: SpinnerDef = SpinnerDef {
    frames: &["☗", "☖"],
    interval_ms: 120,
};

pub const TOGGLE13: SpinnerDef = SpinnerDef {
    frames: &["=", "*", "-"],
    interval_ms: 80,
};

// ============================================================================
// ARROW SPINNERS
// ============================================================================

pub const ARROW: SpinnerDef = SpinnerDef {
    frames: &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
    interval_ms: 100,
};

pub const ARROW2: SpinnerDef = SpinnerDef {
    frames: &["⬆️ ", "↗️ ", "➡️ ", "↘️ ", "⬇️ ", "↙️ ", "⬅️ ", "↖️ "],
    interval_ms: 80,
};

pub const ARROW3: SpinnerDef = SpinnerDef {
    frames: &["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"],
    interval_ms: 120,
};

// ============================================================================
// ANIMATION SPINNERS
// ============================================================================

pub const BOUNCING_BAR: SpinnerDef = SpinnerDef {
    frames: &[
        "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]", "[    ]",
        "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
    ],
    interval_ms: 80,
};

pub const BOUNCING_BALL: SpinnerDef = SpinnerDef {
    frames: &[
        "( ●    )",
        "(  ●   )",
        "(   ●  )",
        "(    ● )",
        "(     ●)",
        "(    ● )",
        "(   ●  )",
        "(  ●   )",
        "( ●    )",
        "(●     )",
    ],
    interval_ms: 80,
};

pub const PONG: SpinnerDef = SpinnerDef {
    frames: &[
        "▐⠂       ▌",
        "▐⠈       ▌",
        "▐ ⠂      ▌",
        "▐ ⠠      ▌",
        "▐  ⡀     ▌",
        "▐  ⠠     ▌",
        "▐   ⠂    ▌",
        "▐   ⠈    ▌",
        "▐    ⠂   ▌",
        "▐    ⠠   ▌",
        "▐     ⡀  ▌",
        "▐     ⠠  ▌",
        "▐      ⠂ ▌",
        "▐      ⠈ ▌",
        "▐       ⠂▌",
        "▐       ⠠▌",
        "▐       ⡀▌",
        "▐      ⠠ ▌",
        "▐      ⠂ ▌",
        "▐     ⠈  ▌",
        "▐     ⠂  ▌",
        "▐    ⠠   ▌",
        "▐    ⡀   ▌",
        "▐   ⠠    ▌",
        "▐   ⠂    ▌",
        "▐  ⠈     ▌",
        "▐  ⠂     ▌",
        "▐ ⠠      ▌",
        "▐ ⡀      ▌",
        "▐⠠       ▌",
    ],
    interval_ms: 80,
};

pub const SHARK: SpinnerDef = SpinnerDef {
    frames: &[
        "▐|\\____________▌",
        "▐_|\\___________▌",
        "▐__|\\__________▌",
        "▐___|\\_________▌",
        "▐____|\\________▌",
        "▐_____|\\_______▌",
        "▐______|\\______▌",
        "▐_______|\\_____▌",
        "▐________|\\____▌",
        "▐_________|\\___▌",
        "▐__________|\\__▌",
        "▐___________|\\_▌",
        "▐____________|\\▌",
        "▐____________/|▌",
        "▐___________/|_▌",
        "▐__________/|__▌",
        "▐_________/|___▌",
        "▐________/|____▌",
        "▐_______/|_____▌",
        "▐______/|______▌",
        "▐_____/|_______▌",
        "▐____/|________▌",
        "▐___/|_________▌",
        "▐__/|__________▌",
        "▐_/|___________▌",
        "▐/|____________▌",
    ],
    interval_ms: 120,
};

pub const BETA_WAVE: SpinnerDef = SpinnerDef {
    frames: &[
        "ρββββββ",
        "βρβββββ",
        "ββρββββ",
        "βββρβββ",
        "ββββρββ",
        "βββββρβ",
        "ββββββρ",
    ],
    interval_ms: 80,
};

pub const AESTHETIC: SpinnerDef = SpinnerDef {
    frames: &[
        "▰▱▱▱▱▱▱",
        "▰▰▱▱▱▱▱",
        "▰▰▰▱▱▱▱",
        "▰▰▰▰▱▱▱",
        "▰▰▰▰▰▱▱",
        "▰▰▰▰▰▰▱",
        "▰▰▰▰▰▰▰",
        "▰▱▱▱▱▱▱",
    ],
    interval_ms: 80,
};

pub const MATERIAL: SpinnerDef = SpinnerDef {
    frames: &[
        "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "████████▁▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "██████████▁▁▁▁▁▁▁▁▁▁",
        "███████████▁▁▁▁▁▁▁▁▁",
        "█████████████▁▁▁▁▁▁▁",
        "██████████████▁▁▁▁▁▁",
        "██████████████▁▁▁▁▁▁",
        "▁██████████████▁▁▁▁▁",
        "▁██████████████▁▁▁▁▁",
        "▁██████████████▁▁▁▁▁",
        "▁▁██████████████▁▁▁▁",
        "▁▁▁██████████████▁▁▁",
        "▁▁▁▁█████████████▁▁▁",
        "▁▁▁▁██████████████▁▁",
        "▁▁▁▁██████████████▁▁",
        "▁▁▁▁▁██████████████▁",
        "▁▁▁▁▁██████████████▁",
        "▁▁▁▁▁██████████████▁",
        "▁▁▁▁▁▁██████████████",
        "▁▁▁▁▁▁██████████████",
        "▁▁▁▁▁▁▁█████████████",
        "▁▁▁▁▁▁▁█████████████",
        "▁▁▁▁▁▁▁▁████████████",
        "▁▁▁▁▁▁▁▁████████████",
        "▁▁▁▁▁▁▁▁▁███████████",
        "▁▁▁▁▁▁▁▁▁███████████",
        "▁▁▁▁▁▁▁▁▁▁██████████",
        "▁▁▁▁▁▁▁▁▁▁██████████",
        "▁▁▁▁▁▁▁▁▁▁▁▁████████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
        "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
        "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
        "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
        "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
        "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
        "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "████████▁▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "█████████▁▁▁▁▁▁▁▁▁▁▁",
        "███████████▁▁▁▁▁▁▁▁▁",
        "████████████▁▁▁▁▁▁▁▁",
        "████████████▁▁▁▁▁▁▁▁",
        "██████████████▁▁▁▁▁▁",
        "██████████████▁▁▁▁▁▁",
        "▁██████████████▁▁▁▁▁",
        "▁██████████████▁▁▁▁▁",
        "▁▁▁█████████████▁▁▁▁",
        "▁▁▁▁▁████████████▁▁▁",
        "▁▁▁▁▁████████████▁▁▁",
        "▁▁▁▁▁▁███████████▁▁▁",
        "▁▁▁▁▁▁▁▁█████████▁▁▁",
        "▁▁▁▁▁▁▁▁█████████▁▁▁",
        "▁▁▁▁▁▁▁▁▁█████████▁▁",
        "▁▁▁▁▁▁▁▁▁█████████▁▁",
        "▁▁▁▁▁▁▁▁▁▁█████████▁",
        "▁▁▁▁▁▁▁▁▁▁▁████████▁",
        "▁▁▁▁▁▁▁▁▁▁▁████████▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
        "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
    ],
    interval_ms: 17,
};

// ============================================================================
// EMOJI SPINNERS
// ============================================================================

pub const CLOCK: SpinnerDef = SpinnerDef {
    frames: &[
        "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
    ],
    interval_ms: 100,
};

pub const MOON: SpinnerDef = SpinnerDef {
    frames: &["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "],
    interval_ms: 80,
};

pub const EARTH: SpinnerDef = SpinnerDef {
    frames: &["🌍 ", "🌎 ", "🌏 "],
    interval_ms: 180,
};

pub const HEARTS: SpinnerDef = SpinnerDef {
    frames: &["💛 ", "💙 ", "💜 ", "💚 ", "💗 "],
    interval_ms: 100,
};

pub const SMILEY: SpinnerDef = SpinnerDef {
    frames: &["😄 ", "😝 "],
    interval_ms: 200,
};

pub const MONKEY: SpinnerDef = SpinnerDef {
    frames: &["🙈 ", "🙈 ", "🙉 ", "🙊 "],
    interval_ms: 300,
};

pub const RUNNER: SpinnerDef = SpinnerDef {
    frames: &["🚶 ", "🏃 "],
    interval_ms: 140,
};

pub const WEATHER: SpinnerDef = SpinnerDef {
    frames: &[
        "☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️ ", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "⛈ ",
        "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️ ", "🌤 ", "☀️ ", "☀️ ",
    ],
    interval_ms: 100,
};

pub const CHRISTMAS: SpinnerDef = SpinnerDef {
    frames: &["🌲", "🎄"],
    interval_ms: 400,
};

pub const GRENADE: SpinnerDef = SpinnerDef {
    frames: &[
        "،  ", "′  ", " ´ ", " ‾ ", "  ⸌", "  ⸊", "  |", "  ⁎", "  ⁕", " ෴ ", "  ⁓", "   ", "   ",
        "   ",
    ],
    interval_ms: 80,
};

pub const FINGER_DANCE: SpinnerDef = SpinnerDef {
    frames: &["🤘 ", "🤟 ", "🖖 ", "✋ ", "🤚 ", "👆 "],
    interval_ms: 160,
};

pub const SPEAKER: SpinnerDef = SpinnerDef {
    frames: &["🔈 ", "🔉 ", "🔊 ", "🔉 "],
    interval_ms: 160,
};

pub const ORANGE_PULSE: SpinnerDef = SpinnerDef {
    frames: &["🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 "],
    interval_ms: 100,
};

pub const BLUE_PULSE: SpinnerDef = SpinnerDef {
    frames: &["🔹 ", "🔷 ", "🔵 ", "🔵 ", "🔷 "],
    interval_ms: 100,
};

pub const ORANGE_BLUE_PULSE: SpinnerDef = SpinnerDef {
    frames: &[
        "🔸 ", "🔶 ", "🟠 ", "🟠 ", "🔶 ", "🔹 ", "🔷 ", "🔵 ", "🔵 ", "🔷 ",
    ],
    interval_ms: 100,
};

pub const TIME_TRAVEL: SpinnerDef = SpinnerDef {
    frames: &[
        "🕛 ", "🕚 ", "🕙 ", "🕘 ", "🕗 ", "🕖 ", "🕕 ", "🕔 ", "🕓 ", "🕒 ", "🕑 ", "🕐 ",
    ],
    interval_ms: 100,
};

pub const MINDBLOWN: SpinnerDef = SpinnerDef {
    frames: &[
        "😐 ", "😐 ", "😮 ", "😮 ", "😦 ", "😦 ", "😧 ", "😧 ", "🤯 ", "💥 ", "✨ ", "　 ", "　 ",
        "　 ",
    ],
    interval_ms: 160,
};

// ============================================================================
// MISC SPINNERS
// ============================================================================

pub const DQPB: SpinnerDef = SpinnerDef {
    frames: &["d", "q", "p", "b"],
    interval_ms: 100,
};

pub const POINT: SpinnerDef = SpinnerDef {
    frames: &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
    interval_ms: 125,
};

pub const LAYER: SpinnerDef = SpinnerDef {
    frames: &["-", "=", "≡"],
    interval_ms: 150,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_defs_have_frames_and_intervals() {
        // Every spinner definition must have at least one frame and a
        // strictly positive interval (frame math divides by it).
        let defs = [
            DOTS, DOTS2, DOTS3, DOTS4, DOTS5, DOTS6, DOTS7, DOTS8, DOTS9, DOTS10, LINE, LINE2, PIPE,
        ];
        for (i, def) in defs.iter().enumerate() {
            assert!(!def.frames.is_empty(), "spinner #{i} has no frames");
            assert!(def.interval_ms > 0, "spinner #{i} has zero interval");
        }
    }
}
