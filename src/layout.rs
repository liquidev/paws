//! Layouting types.

#![warn(missing_copy_implementations)]

/// Group layout type. This defines how subgroups are arranged inside of a group.
#[derive(Copy, Clone, PartialEq)]
pub enum Layout {
    /// The layout for individual subgroups is defined by the user via `ui.set_cursor(x, y)`.
    Freeform,
    /// Subgroups are laid out horizontally, from left to right. The default starting point for layout is the upper-left
    /// corner of the group.
    Horizontal,
    /// Subgroups are laid out vertically, from top to bottom. The default starting point for layout is the upper-left
    /// corner of the group.
    Vertical,
    /// Subgroups are laid out horizontally, from right to left. The default starting point for layout is the
    /// upper-right corner of the group.
    HorizontalRev,
    /// Subgroups are laid out vertically, from bottom to top. The default starting point for layout is the
    /// lower-left corner of the group.
    VerticalRev,
}

/// Horizontal alignment position.
#[derive(Copy, Clone, PartialEq)]
pub enum AlignH {
    Left,
    Center,
    Right,
}

/// Vertical alignment position.
#[derive(Copy, Clone, PartialEq)]
pub enum AlignV {
    Top,
    Middle,
    Bottom,
}

pub use AlignH::*;
pub use AlignV::*;

/// Alignment type. This is used in `ui.align(alignment)` and text rendering.
pub type Alignment = (AlignH, AlignV);

/// Convenience const for `(Center, Middle)` alignment.
pub const CENTER: Alignment = (Center, Middle);

/// Padding amounts.
///
/// Usually you don't need to construct this directly, as this implements From for several types, and paws
/// accepts `impl Into<Padding>` instead of just `Padding` in all functions.
#[derive(Copy, Clone, PartialEq)]
pub struct Padding {
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
    pub top: f32,
}

impl Padding {
    /// Creates padding from horizontal and vertical amounts. This is equivalent constructing
    /// the padding where `right` and `left` are set to `horizontal`, and `bottom` and `top` are
    /// set to `vertical`.
    pub fn hv(horizontal: f32, vertical: f32) -> Self {
        Self {
            right: horizontal,
            bottom: vertical,
            left: horizontal,
            top: vertical,
        }
    }

    /// Creates an even amount of padding for all sides.
    pub fn even(amount: f32) -> Self {
        Self {
            right: amount,
            bottom: amount,
            left: amount,
            top: amount,
        }
    }

    /// Creates the given amount of right padding. All other sides are set to `0.0`.
    pub fn right(amount: f32) -> Self {
        Self {
            right: amount,
            ..Self::default()
        }
    }

    /// Creates the given amount of bottom padding. All other sides are set to `0.0`.
    pub fn bottom(amount: f32) -> Self {
        Self {
            bottom: amount,
            ..Self::default()
        }
    }

    /// Creates the given amount of left padding. All other sides are set to `0.0`.
    pub fn left(amount: f32) -> Self {
        Self {
            left: amount,
            ..Self::default()
        }
    }

    /// Creates the given amount of top padding. All other sides are set to `0.0`.
    pub fn top(amount: f32) -> Self {
        Self {
            top: amount,
            ..Self::default()
        }
    }
}

impl Default for Padding {
    /// The default amount of padding is `0.0` for all sides.
    fn default() -> Self {
        Self {
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
            top: 0.0,
        }
    }
}

impl From<(f32, f32)> for Padding {
    /// Creates padding from horizontal and vertical amounts. This is the same as calling
    /// `Padding::hv(horizontal, vertical)`.
    fn from((horizontal, vertical): (f32, f32)) -> Self {
        Self::hv(horizontal, vertical)
    }
}

impl From<f32> for Padding {
    /// Creates the given amount of padding for all sides. This is the same as calling `Padding::even(amount)`.
    fn from(amount: f32) -> Self {
        Self::even(amount)
    }
}
