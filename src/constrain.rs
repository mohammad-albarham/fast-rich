//! Constrain a renderable to a maximum width (Rich's `Constrain`).

use crate::console::RenderContext;
use crate::renderable::{BoxedRenderable, Renderable, Segment};

/// Renders its child no wider than `width` cells.
///
/// ```
/// use fast_rich::constrain::Constrain;
/// use fast_rich::renderable::render_lines;
/// use fast_rich::style::Style;
/// use fast_rich::text::Text;
/// let c = Constrain::new(Text::plain("aaa bbb ccc"), Some(7));
/// assert_eq!(render_lines(&c, 40, Style::new()).len(), 2);
/// ```
pub struct Constrain {
    child: BoxedRenderable,
    width: Option<usize>,
}

impl Constrain {
    /// Constrain `child` to at most `width` cells (`None`: no limit).
    pub fn new(child: impl Renderable + Send + Sync + 'static, width: Option<usize>) -> Self {
        Constrain {
            child: Box::new(child),
            width,
        }
    }
}

impl Renderable for Constrain {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        let width = self.width.map_or(context.width, |w| w.min(context.width));
        self.child.render(&RenderContext {
            width,
            height: context.height,
        })
    }

    fn min_width(&self) -> usize {
        let min = self.child.min_width();
        self.width.map_or(min, |w| min.min(w))
    }

    fn max_width(&self) -> usize {
        let max = self.child.max_width();
        self.width.map_or(max, |w| max.min(w))
    }
}
