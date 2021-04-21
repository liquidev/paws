//! Abstract renderer trait. Required if you want any of the extra rendering functions to work.

use crate::common::*;
use crate::layout::*;

/// The type of line cap to use when rendering.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Square,
    Round,
}

/// The type of line joint to use when rendering.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LineJoint {
    Bevel,
    Miter,
    Round,
}

/// The renderer trait, used for all things drawing-related.
/// This trait is available for the user's convenience. If a type that doesn't implement `Renderer` is specified in
/// `Ui`, convenience rendering functions are not available and the user must handle rendering all by themselves.
///
/// ## A note on rendering lines
///
/// Renderers should try their best to make lines pixel-perfect. What this means is that lines with a thickness of
/// 1.0 shouldn't get placed inbetween pixels, as that will make it look blurred out. Some renderers do that, and on
/// those renderers stroke points should get moved by half a pixel.
pub trait Renderer {
    /// The font type used for rendering text. May be `()` if text rendering isn't supported.
    type Font;
    /// The image type used for rendering images. May be `()` if text rendering isn't supported.
    type Image: SizedImage;

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
    /// Draws a line from point A to point B, with the given color, cap type, joint type, and thickness.
    fn line(
        &mut self,
        a: Point,
        b: Point,
        color: Color,
        cap: LineCap,
        joint: LineJoint,
        thickness: f32,
    );

    /// Draws text aligned inside of the provided rectangle, with the given color.
    fn text(
        &mut self,
        rect: Rect,
        font: &Self::Font,
        text: &str,
        color: Color,
        alignment: Alignment,
    );

    /// Draws an image. The renderer should at least take the rectangle's position into account, but things like
    /// wrapping mode and sampling are not specified by this library.
    fn image(&mut self, rect: Rect, image: &Self::Image);
}

/// An image whose size can be measured.
pub trait SizedImage {
    /// Returns the size of the image.
    fn size(&self) -> Vector;
}

/// A dummy renderer. This can be used for executing graphics commands without a graphical backend available.
pub struct NoRenderer;

/// A dummy font used by the NoRenderer backend.
pub struct NoRendererFont;

/// A dummy image used by the NoRenderer backend.
pub struct NoRendererImage;

impl Renderer for NoRenderer {
    type Font = NoRendererFont;
    type Image = NoRendererImage;

    fn push(&mut self) {}
    fn pop(&mut self) {}
    fn translate(&mut self, _: Vector) {}
    fn clip(&mut self, _: Rect) {}

    fn fill(&mut self, _: Rect, _: Color, _: f32) {}
    fn outline(&mut self, _: Rect, _: Color, _: f32, _: f32) {}
    fn line(&mut self, _: Point, _: Point, _: Color, _: LineCap, _: LineJoint, _: f32) {}

    fn text(&mut self, _: Rect, _: &Self::Font, _: &str, _: Color, _: Alignment) {}
    fn image(&mut self, _: Rect, _: &Self::Image) {}
}

impl SizedImage for NoRendererImage {
    fn size(&self) -> Vector {
        Vector::default()
    }
}
