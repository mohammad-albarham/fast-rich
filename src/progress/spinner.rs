//! Spinner animations for indeterminate progress.
//!
//! Provides 85 spinner styles from cli-spinners.
//!
//! # Attribution
//! Spinner definitions are sourced from cli-spinners:
//! MIT License - Copyright (c) Sindre Sorhus <sindresorhus@gmail.com>
//! https://github.com/sindresorhus/cli-spinners

use super::spinner_data::*;
use crate::style::{Color, Style};
use crate::text::Span;
use std::time::Instant;

/// Spinner animation style.
///
/// Use `SpinnerStyle::from_name()` for string-based lookup (Python parity).
///
/// # Examples
/// ```
/// use fast_rich::progress::SpinnerStyle;
///
/// // Using enum directly
/// let style = SpinnerStyle::Dots;
///
/// // Using string lookup
/// let style = SpinnerStyle::from_name("moon").unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerStyle {
    // ==================== BRAILLE DOTS ====================
    /// Default dots animation (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
    #[default]
    Dots,
    /// Dots variant 2
    Dots2,
    /// Dots variant 3
    Dots3,
    /// Dots variant 4
    Dots4,
    /// Dots variant 5
    Dots5,
    /// Dots variant 6
    Dots6,
    /// Dots variant 7
    Dots7,
    /// Dots variant 8
    Dots8,
    /// Dots variant 9
    Dots9,
    /// Dots variant 10
    Dots10,
    /// Dots variant 11
    Dots11,
    /// Dots variant 12 (two-character)
    Dots12,
    /// Dots variant 13
    Dots13,
    /// Dots variant 14
    Dots14,
    /// Circular dots pattern
    DotsCircle,
    /// Sand falling animation
    Sand,
    /// Bounce animation
    Bounce,

    // ==================== LINES ====================
    /// Classic line spinner (-\|/)
    Line,
    /// Line variant 2
    Line2,
    /// Pipe corners
    Pipe,
    /// Rolling line
    RollingLine,
    /// Simple dots (...)
    SimpleDots,
    /// Simple dots scrolling
    SimpleDotsScrolling,

    // ==================== STARS ====================
    /// Star animation
    Star,
    /// Star variant 2
    Star2,

    // ==================== SHAPES ====================
    /// Arc animation
    Arc,
    /// Circle animation
    Circle,
    /// Circle halves
    CircleHalves,
    /// Circle quarters
    CircleQuarters,
    /// Square corners
    SquareCorners,
    /// Triangle animation
    Triangle,
    /// Binary sequence
    Binary,
    /// Squish animation
    Squish,
    /// Flip animation
    Flip,
    /// Hamburger menu animation
    Hamburger,

    // ==================== BOXES ====================
    /// Box bounce
    BoxBounce,
    /// Box bounce variant 2
    BoxBounce2,
    /// Noise animation
    Noise,

    // ==================== GROWING ====================
    /// Vertical growing bar
    GrowVertical,
    /// Horizontal growing bar
    GrowHorizontal,
    /// Balloon animation
    Balloon,
    /// Balloon variant 2
    Balloon2,

    // ==================== TOGGLES ====================
    /// Toggle animation
    Toggle,
    /// Toggle variant 2
    Toggle2,
    /// Toggle variant 3
    Toggle3,
    /// Toggle variant 4
    Toggle4,
    /// Toggle variant 5
    Toggle5,
    /// Toggle variant 6
    Toggle6,
    /// Toggle variant 7
    Toggle7,
    /// Toggle variant 8
    Toggle8,
    /// Toggle variant 9
    Toggle9,
    /// Toggle variant 10
    Toggle10,
    /// Toggle variant 11
    Toggle11,
    /// Toggle variant 12
    Toggle12,
    /// Toggle variant 13
    Toggle13,

    // ==================== ARROWS ====================
    /// Arrow animation
    Arrow,
    /// Arrow variant 2 (emoji)
    Arrow2,
    /// Arrow variant 3
    Arrow3,

    // ==================== ANIMATIONS ====================
    /// Bouncing bar animation
    BouncingBar,
    /// Bouncing ball animation
    BouncingBall,
    /// Pong game animation
    Pong,
    /// Shark animation
    Shark,
    /// Beta wave animation
    BetaWave,
    /// Aesthetic loading bar
    Aesthetic,
    /// Material design animation
    Material,

    // ==================== EMOJI ====================
    /// Clock animation 🕐
    Clock,
    /// Moon phases 🌑🌒🌓🌔🌕
    Moon,
    /// Earth rotation 🌍🌎🌏
    Earth,
    /// Hearts 💛💙💜💚
    Hearts,
    /// Smiley 😄
    Smiley,
    /// See no evil monkey 🙈🙉🙊
    Monkey,
    /// Runner 🚶🏃
    Runner,
    /// Weather animation ☀️🌧
    Weather,
    /// Christmas tree 🌲🎄
    Christmas,
    /// Grenade explosion
    Grenade,
    /// Finger dance 🤘🤟
    FingerDance,
    /// Speaker 🔈🔉🔊
    Speaker,
    /// Orange pulse 🔸🔶🟠
    OrangePulse,
    /// Blue pulse 🔹🔷🔵
    BluePulse,
    /// Orange-blue pulse
    OrangeBluePulse,
    /// Time travel (reverse clock)
    TimeTravel,
    /// Mind blown 🤯
    Mindblown,

    // ==================== MISC ====================
    /// dqpb animation
    Dqpb,
    /// Point animation
    Point,
    /// Layer animation
    Layer,
}

impl SpinnerStyle {
    /// Get the spinner definition (frames and interval).
    pub fn def(&self) -> &'static SpinnerDef {
        match self {
            // Braille dots
            SpinnerStyle::Dots => &DOTS,
            SpinnerStyle::Dots2 => &DOTS2,
            SpinnerStyle::Dots3 => &DOTS3,
            SpinnerStyle::Dots4 => &DOTS4,
            SpinnerStyle::Dots5 => &DOTS5,
            SpinnerStyle::Dots6 => &DOTS6,
            SpinnerStyle::Dots7 => &DOTS7,
            SpinnerStyle::Dots8 => &DOTS8,
            SpinnerStyle::Dots9 => &DOTS9,
            SpinnerStyle::Dots10 => &DOTS10,
            SpinnerStyle::Dots11 => &DOTS11,
            SpinnerStyle::Dots12 => &DOTS12,
            SpinnerStyle::Dots13 => &DOTS13,
            SpinnerStyle::Dots14 => &DOTS14,
            SpinnerStyle::DotsCircle => &DOTS_CIRCLE,
            SpinnerStyle::Sand => &SAND,
            SpinnerStyle::Bounce => &BOUNCE,

            // Lines
            SpinnerStyle::Line => &LINE,
            SpinnerStyle::Line2 => &LINE2,
            SpinnerStyle::Pipe => &PIPE,
            SpinnerStyle::RollingLine => &ROLLING_LINE,
            SpinnerStyle::SimpleDots => &SIMPLE_DOTS,
            SpinnerStyle::SimpleDotsScrolling => &SIMPLE_DOTS_SCROLLING,

            // Stars
            SpinnerStyle::Star => &STAR,
            SpinnerStyle::Star2 => &STAR2,

            // Shapes
            SpinnerStyle::Arc => &ARC,
            SpinnerStyle::Circle => &CIRCLE,
            SpinnerStyle::CircleHalves => &CIRCLE_HALVES,
            SpinnerStyle::CircleQuarters => &CIRCLE_QUARTERS,
            SpinnerStyle::SquareCorners => &SQUARE_CORNERS,
            SpinnerStyle::Triangle => &TRIANGLE,
            SpinnerStyle::Binary => &BINARY,
            SpinnerStyle::Squish => &SQUISH,
            SpinnerStyle::Flip => &FLIP,
            SpinnerStyle::Hamburger => &HAMBURGER,

            // Boxes
            SpinnerStyle::BoxBounce => &BOX_BOUNCE,
            SpinnerStyle::BoxBounce2 => &BOX_BOUNCE2,
            SpinnerStyle::Noise => &NOISE,

            // Growing
            SpinnerStyle::GrowVertical => &GROW_VERTICAL,
            SpinnerStyle::GrowHorizontal => &GROW_HORIZONTAL,
            SpinnerStyle::Balloon => &BALLOON,
            SpinnerStyle::Balloon2 => &BALLOON2,

            // Toggles
            SpinnerStyle::Toggle => &TOGGLE,
            SpinnerStyle::Toggle2 => &TOGGLE2,
            SpinnerStyle::Toggle3 => &TOGGLE3,
            SpinnerStyle::Toggle4 => &TOGGLE4,
            SpinnerStyle::Toggle5 => &TOGGLE5,
            SpinnerStyle::Toggle6 => &TOGGLE6,
            SpinnerStyle::Toggle7 => &TOGGLE7,
            SpinnerStyle::Toggle8 => &TOGGLE8,
            SpinnerStyle::Toggle9 => &TOGGLE9,
            SpinnerStyle::Toggle10 => &TOGGLE10,
            SpinnerStyle::Toggle11 => &TOGGLE11,
            SpinnerStyle::Toggle12 => &TOGGLE12,
            SpinnerStyle::Toggle13 => &TOGGLE13,

            // Arrows
            SpinnerStyle::Arrow => &ARROW,
            SpinnerStyle::Arrow2 => &ARROW2,
            SpinnerStyle::Arrow3 => &ARROW3,

            // Animations
            SpinnerStyle::BouncingBar => &BOUNCING_BAR,
            SpinnerStyle::BouncingBall => &BOUNCING_BALL,
            SpinnerStyle::Pong => &PONG,
            SpinnerStyle::Shark => &SHARK,
            SpinnerStyle::BetaWave => &BETA_WAVE,
            SpinnerStyle::Aesthetic => &AESTHETIC,
            SpinnerStyle::Material => &MATERIAL,

            // Emoji
            SpinnerStyle::Clock => &CLOCK,
            SpinnerStyle::Moon => &MOON,
            SpinnerStyle::Earth => &EARTH,
            SpinnerStyle::Hearts => &HEARTS,
            SpinnerStyle::Smiley => &SMILEY,
            SpinnerStyle::Monkey => &MONKEY,
            SpinnerStyle::Runner => &RUNNER,
            SpinnerStyle::Weather => &WEATHER,
            SpinnerStyle::Christmas => &CHRISTMAS,
            SpinnerStyle::Grenade => &GRENADE,
            SpinnerStyle::FingerDance => &FINGER_DANCE,
            SpinnerStyle::Speaker => &SPEAKER,
            SpinnerStyle::OrangePulse => &ORANGE_PULSE,
            SpinnerStyle::BluePulse => &BLUE_PULSE,
            SpinnerStyle::OrangeBluePulse => &ORANGE_BLUE_PULSE,
            SpinnerStyle::TimeTravel => &TIME_TRAVEL,
            SpinnerStyle::Mindblown => &MINDBLOWN,

            // Misc
            SpinnerStyle::Dqpb => &DQPB,
            SpinnerStyle::Point => &POINT,
            SpinnerStyle::Layer => &LAYER,
        }
    }

    /// Get the frames for this spinner style.
    pub fn frames(&self) -> &'static [&'static str] {
        self.def().frames
    }

    /// Get the interval between frames in milliseconds.
    pub fn interval_ms(&self) -> u64 {
        self.def().interval_ms
    }

    /// Create a SpinnerStyle from a string name (Python parity).
    ///
    /// Names are case-insensitive and support both camelCase and snake_case.
    ///
    /// # Examples
    /// ```
    /// use fast_rich::progress::SpinnerStyle;
    ///
    /// assert!(SpinnerStyle::from_name("dots").is_some());
    /// assert!(SpinnerStyle::from_name("moon").is_some());
    /// assert!(SpinnerStyle::from_name("bouncingBar").is_some());
    /// assert!(SpinnerStyle::from_name("bouncing_bar").is_some());
    /// ```
    pub fn from_name(name: &str) -> Option<SpinnerStyle> {
        let name_lower = name.to_lowercase().replace('_', "");
        match name_lower.as_str() {
            // Braille dots
            "dots" => Some(SpinnerStyle::Dots),
            "dots2" => Some(SpinnerStyle::Dots2),
            "dots3" => Some(SpinnerStyle::Dots3),
            "dots4" => Some(SpinnerStyle::Dots4),
            "dots5" => Some(SpinnerStyle::Dots5),
            "dots6" => Some(SpinnerStyle::Dots6),
            "dots7" => Some(SpinnerStyle::Dots7),
            "dots8" => Some(SpinnerStyle::Dots8),
            "dots9" => Some(SpinnerStyle::Dots9),
            "dots10" => Some(SpinnerStyle::Dots10),
            "dots11" => Some(SpinnerStyle::Dots11),
            "dots12" => Some(SpinnerStyle::Dots12),
            "dots13" => Some(SpinnerStyle::Dots13),
            "dots14" => Some(SpinnerStyle::Dots14),
            "dotscircle" => Some(SpinnerStyle::DotsCircle),
            "sand" => Some(SpinnerStyle::Sand),
            "bounce" => Some(SpinnerStyle::Bounce),

            // Lines
            "line" => Some(SpinnerStyle::Line),
            "line2" => Some(SpinnerStyle::Line2),
            "pipe" => Some(SpinnerStyle::Pipe),
            "rollingline" => Some(SpinnerStyle::RollingLine),
            "simpledots" => Some(SpinnerStyle::SimpleDots),
            "simpledotsscrolling" => Some(SpinnerStyle::SimpleDotsScrolling),

            // Stars
            "star" => Some(SpinnerStyle::Star),
            "star2" => Some(SpinnerStyle::Star2),

            // Shapes
            "arc" => Some(SpinnerStyle::Arc),
            "circle" => Some(SpinnerStyle::Circle),
            "circlehalves" => Some(SpinnerStyle::CircleHalves),
            "circlequarters" => Some(SpinnerStyle::CircleQuarters),
            "squarecorners" => Some(SpinnerStyle::SquareCorners),
            "triangle" => Some(SpinnerStyle::Triangle),
            "binary" => Some(SpinnerStyle::Binary),
            "squish" => Some(SpinnerStyle::Squish),
            "flip" => Some(SpinnerStyle::Flip),
            "hamburger" => Some(SpinnerStyle::Hamburger),

            // Boxes
            "boxbounce" => Some(SpinnerStyle::BoxBounce),
            "boxbounce2" => Some(SpinnerStyle::BoxBounce2),
            "noise" => Some(SpinnerStyle::Noise),

            // Growing
            "growvertical" => Some(SpinnerStyle::GrowVertical),
            "growhorizontal" => Some(SpinnerStyle::GrowHorizontal),
            "balloon" => Some(SpinnerStyle::Balloon),
            "balloon2" => Some(SpinnerStyle::Balloon2),

            // Toggles
            "toggle" => Some(SpinnerStyle::Toggle),
            "toggle2" => Some(SpinnerStyle::Toggle2),
            "toggle3" => Some(SpinnerStyle::Toggle3),
            "toggle4" => Some(SpinnerStyle::Toggle4),
            "toggle5" => Some(SpinnerStyle::Toggle5),
            "toggle6" => Some(SpinnerStyle::Toggle6),
            "toggle7" => Some(SpinnerStyle::Toggle7),
            "toggle8" => Some(SpinnerStyle::Toggle8),
            "toggle9" => Some(SpinnerStyle::Toggle9),
            "toggle10" => Some(SpinnerStyle::Toggle10),
            "toggle11" => Some(SpinnerStyle::Toggle11),
            "toggle12" => Some(SpinnerStyle::Toggle12),
            "toggle13" => Some(SpinnerStyle::Toggle13),

            // Arrows
            "arrow" => Some(SpinnerStyle::Arrow),
            "arrow2" => Some(SpinnerStyle::Arrow2),
            "arrow3" => Some(SpinnerStyle::Arrow3),

            // Animations
            "bouncingbar" => Some(SpinnerStyle::BouncingBar),
            "bouncingball" => Some(SpinnerStyle::BouncingBall),
            "pong" => Some(SpinnerStyle::Pong),
            "shark" => Some(SpinnerStyle::Shark),
            "betawave" => Some(SpinnerStyle::BetaWave),
            "aesthetic" => Some(SpinnerStyle::Aesthetic),
            "material" => Some(SpinnerStyle::Material),

            // Emoji
            "clock" => Some(SpinnerStyle::Clock),
            "moon" => Some(SpinnerStyle::Moon),
            "earth" => Some(SpinnerStyle::Earth),
            "hearts" => Some(SpinnerStyle::Hearts),
            "smiley" => Some(SpinnerStyle::Smiley),
            "monkey" => Some(SpinnerStyle::Monkey),
            "runner" => Some(SpinnerStyle::Runner),
            "weather" => Some(SpinnerStyle::Weather),
            "christmas" => Some(SpinnerStyle::Christmas),
            "grenade" => Some(SpinnerStyle::Grenade),
            "fingerdance" => Some(SpinnerStyle::FingerDance),
            "speaker" => Some(SpinnerStyle::Speaker),
            "orangepulse" => Some(SpinnerStyle::OrangePulse),
            "bluepulse" => Some(SpinnerStyle::BluePulse),
            "orangebluepulse" => Some(SpinnerStyle::OrangeBluePulse),
            "timetravel" => Some(SpinnerStyle::TimeTravel),
            "mindblown" => Some(SpinnerStyle::Mindblown),

            // Misc
            "dqpb" => Some(SpinnerStyle::Dqpb),
            "point" => Some(SpinnerStyle::Point),
            "layer" => Some(SpinnerStyle::Layer),

            _ => None,
        }
    }

    /// Get all available spinner style names.
    pub fn all_names() -> &'static [&'static str] {
        &[
            "dots",
            "dots2",
            "dots3",
            "dots4",
            "dots5",
            "dots6",
            "dots7",
            "dots8",
            "dots9",
            "dots10",
            "dots11",
            "dots12",
            "dots13",
            "dots14",
            "dotsCircle",
            "sand",
            "bounce",
            "line",
            "line2",
            "pipe",
            "rollingLine",
            "simpleDots",
            "simpleDotsScrolling",
            "star",
            "star2",
            "arc",
            "circle",
            "circleHalves",
            "circleQuarters",
            "squareCorners",
            "triangle",
            "binary",
            "squish",
            "flip",
            "hamburger",
            "boxBounce",
            "boxBounce2",
            "noise",
            "growVertical",
            "growHorizontal",
            "balloon",
            "balloon2",
            "toggle",
            "toggle2",
            "toggle3",
            "toggle4",
            "toggle5",
            "toggle6",
            "toggle7",
            "toggle8",
            "toggle9",
            "toggle10",
            "toggle11",
            "toggle12",
            "toggle13",
            "arrow",
            "arrow2",
            "arrow3",
            "bouncingBar",
            "bouncingBall",
            "pong",
            "shark",
            "betaWave",
            "aesthetic",
            "material",
            "clock",
            "moon",
            "earth",
            "hearts",
            "smiley",
            "monkey",
            "runner",
            "weather",
            "christmas",
            "grenade",
            "fingerDance",
            "speaker",
            "orangePulse",
            "bluePulse",
            "orangeBluePulse",
            "timeTravel",
            "mindblown",
            "dqpb",
            "point",
            "layer",
        ]
    }
}

/// A spinner for indeterminate progress.
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Spinner style
    style: SpinnerStyle,
    /// Start time for animation
    start_time: Instant,
    /// Text to display after the spinner
    text: String,
    /// Style for the spinner character
    spinner_style: Style,
    /// Style for the text
    text_style: Style,
}

impl Spinner {
    /// Create a new spinner with optional text.
    pub fn new(text: &str) -> Self {
        Spinner {
            style: SpinnerStyle::Dots,
            start_time: Instant::now(),
            text: text.to_string(),
            spinner_style: Style::new().foreground(Color::Cyan),
            text_style: Style::new(),
        }
    }

    /// Set the spinner style.
    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the spinner style by name (Python parity).
    ///
    /// Returns `None` if the name is not recognized.
    pub fn style_name(mut self, name: &str) -> Option<Self> {
        SpinnerStyle::from_name(name).map(|s| {
            self.style = s;
            self
        })
    }

    /// Set the spinner character style.
    pub fn spinner_style(mut self, style: Style) -> Self {
        self.spinner_style = style;
        self
    }

    /// Set the text style.
    pub fn text_style(mut self, style: Style) -> Self {
        self.text_style = style;
        self
    }

    /// Set the text.
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Update the text.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    /// Get the text.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Get the spinner style.
    pub fn get_style(&self) -> SpinnerStyle {
        self.style
    }

    /// Get the current frame index.
    fn current_frame_index(&self) -> usize {
        let elapsed_ms = self.start_time.elapsed().as_millis() as u64;
        let interval = self.style.interval_ms();
        let frames = self.style.frames();
        ((elapsed_ms / interval) as usize) % frames.len()
    }

    /// Get the current frame character.
    pub fn current_frame(&self) -> &'static str {
        let frames = self.style.frames();
        let idx = self.current_frame_index();
        frames[idx]
    }

    /// Render the spinner to spans.
    pub fn render(&self) -> Vec<Span> {
        vec![
            Span::styled(self.current_frame().to_string(), self.spinner_style),
            Span::raw(" "),
            Span::styled(self.text.clone(), self.text_style),
        ]
    }

    /// Render to a string (for simple output).
    pub fn to_string_colored(&self) -> String {
        format!("{} {}", self.current_frame(), self.text)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Spinner::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_frames() {
        let style = SpinnerStyle::Dots;
        let frames = style.frames();
        assert!(!frames.is_empty());
        assert_eq!(frames[0], "⠋");
    }

    #[test]
    fn test_spinner_render() {
        let spinner = Spinner::new("Loading...");
        let spans = spinner.render();
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn test_all_spinner_styles_have_frames() {
        for name in SpinnerStyle::all_names() {
            let style = SpinnerStyle::from_name(name)
                .unwrap_or_else(|| panic!("Failed to find style: {}", name));
            let frames = style.frames();
            assert!(!frames.is_empty(), "{} has no frames", name);
            assert!(style.interval_ms() > 0, "{} has invalid interval", name);
        }
    }

    #[test]
    fn test_spinner_from_name() {
        // Test various names
        assert!(SpinnerStyle::from_name("dots").is_some());
        assert!(SpinnerStyle::from_name("Dots").is_some());
        assert!(SpinnerStyle::from_name("DOTS").is_some());
        assert!(SpinnerStyle::from_name("moon").is_some());
        assert!(SpinnerStyle::from_name("bouncingBar").is_some());
        assert!(SpinnerStyle::from_name("bouncing_bar").is_some());
        assert!(SpinnerStyle::from_name("invalid_name").is_none());
    }

    #[test]
    fn test_emoji_spinners() {
        let clock = SpinnerStyle::Clock;
        assert!(clock.frames().iter().any(|f| f.contains("🕐")));

        let moon = SpinnerStyle::Moon;
        assert!(moon.frames().iter().any(|f| f.contains("🌕")));
    }
}
