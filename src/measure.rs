//! Measure API for calculating renderable dimensions.
//!
//! Allows measuring how a renderable will be rendered without actually rendering it.

use crate::console::RenderContext;
use crate::renderable::Renderable;

/// Measurement of a renderable's dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement {
    /// Minimum width required
    pub minimum: usize,
    /// Maximum width that will be used
    pub maximum: usize,
    /// Number of lines when rendered
    pub lines: usize,
}

impl Measurement {
    /// Create a new measurement.
    pub fn new(minimum: usize, maximum: usize, lines: usize) -> Self {
        Measurement {
            minimum,
            maximum,
            lines,
        }
    }

    /// Measure a renderable at a given width.
    pub fn measure(renderable: &impl Renderable, width: usize) -> Self {
        let context = RenderContext { width };
        let segments = renderable.render(&context);

        let lines = segments.len();
        let minimum = renderable.min_width();
        let maximum = renderable.max_width().min(width);

        Measurement {
            minimum,
            maximum,
            lines,
        }
    }

    /// Get the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f64 {
        if self.lines == 0 {
            0.0
        } else {
            self.maximum as f64 / self.lines as f64
        }
    }

    /// Check if the measurement fits within given dimensions.
    pub fn fits(&self, width: usize, height: usize) -> bool {
        self.minimum <= width && self.lines <= height
    }

    /// Get the total area (width * height).
    pub fn area(&self) -> usize {
        self.maximum * self.lines
    }
}

/// Helper trait for measuring renderables.
pub trait Measurable {
    /// Measure this renderable at the given width.
    fn measure(&self, width: usize) -> Measurement;
}

impl<T: Renderable> Measurable for T {
    fn measure(&self, width: usize) -> Measurement {
        Measurement::measure(self, width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::Text;

    #[test]
    fn test_measurement_creation() {
        let m = Measurement::new(10, 50, 3);
        assert_eq!(m.minimum, 10);
        assert_eq!(m.maximum, 50);
        assert_eq!(m.lines, 3);
    }

    #[test]
    fn test_measure_text() {
        let text = Text::plain("Hello, World!");
        let m = Measurement::measure(&text, 80);

        assert!(m.minimum > 0);
        assert!(m.lines > 0);
    }

    #[test]
    fn test_aspect_ratio() {
        let m = Measurement::new(20, 40, 5);
        assert_eq!(m.aspect_ratio(), 8.0);
    }

    #[test]
    fn test_fits() {
        let m = Measurement::new(10, 40, 3);
        assert!(m.fits(50, 5));
        assert!(!m.fits(5, 5)); // Too narrow
        assert!(!m.fits(50, 2)); // Too short
    }

    #[test]
    fn test_area() {
        let m = Measurement::new(10, 20, 5);
        assert_eq!(m.area(), 100); // 20 * 5
    }
}
