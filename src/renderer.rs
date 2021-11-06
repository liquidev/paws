//! Abstract renderer trait. Required if you want any of the extra rendering functions to work.

use crate::common::*;
use crate::layout::*;

/// The type of line cap to use when rendering.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LineCap {
    /// The ends are not extended.
    Butt,
    /// The ends are extended by the line's thickness divided by two.
    Square,
    /// The ends are extended by rounding them off with semicircles.
    Round,
}

/// The renderer trait, used for all things drawing-related.
///
/// ## A note on rendering lines
///
/// Renderers should try their best to make lines pixel-perfect. What this means is that lines with a thickness of
/// 1.0 shouldn't get placed inbetween pixels, as that will make it look blurred out. Some vector graphics renderers
/// do that, and on those renderers stroke points should get moved by half a pixel.
///
/// Examples of such renderers include HTML5 canvas, Cairo, Skia.
pub trait Renderer {
    /// The font type used for rendering text. May be `()` if text rendering isn't supported.
    type Font;

    /// Pushes the current transform matrix and clip region onto a stack.
    fn push(&mut self);
    /// Pops the topmost transform matrix and clip region off the stack and overwrites the current transform matrix
    /// with it.
    fn pop(&mut self);
    /// Translates the transform matrix by the given vector.
    fn translate(&mut self, vec: Vector);
    /// Updates the clip region to the intersection of the current clip region and the provided rectangle.
    /// Initially, the clip region spans the whole window. This only allows for shrinking the clip region in size.
    /// The only way to increase its size is to use `push()` and `pop()`.
    ///
    /// The rectangle should be subject to translation.
    fn clip(&mut self, rect: Rect);

    /// Draws a fill for the provided rectangle, with the given color and corner radius.
    fn fill(&mut self, rect: Rect, color: Color, radius: f32);
    /// Draws an outline for the provided rectangle, with the given color, corner radius, and thickness.
    fn outline(&mut self, rect: Rect, color: Color, radius: f32, thickness: f32);
    /// Draws a line from point A to point B, with the given color, cap type, and thickness.
    fn line(&mut self, a: Point, b: Point, color: Color, cap: LineCap, thickness: f32);

    /// Draws text aligned inside of the provided rectangle, with the given color.
    ///
    /// Returns the horizontal advance of the text.
    fn text(
        &mut self,
        rect: Rect,
        font: &Self::Font,
        text: &str,
        color: Color,
        alignment: Alignment,
    ) -> f32;
}

/// A dummy renderer. This can be used for executing graphics commands without a graphical backend available.
pub struct NoRenderer;

/// A dummy font used by the NoRenderer backend.
pub struct NoRendererFont;

impl Renderer for NoRenderer {
    type Font = NoRendererFont;

    fn push(&mut self) {}
    fn pop(&mut self) {}
    fn translate(&mut self, _: Vector) {}
    fn clip(&mut self, _: Rect) {}

    fn fill(&mut self, _: Rect, _: Color, _: f32) {}
    fn outline(&mut self, _: Rect, _: Color, _: f32, _: f32) {}
    fn line(&mut self, _: Point, _: Point, _: Color, _: LineCap, _: f32) {}

    fn text(&mut self, _: Rect, _: &Self::Font, _: &str, _: Color, _: Alignment) -> f32 {
        0.0
    }
}
