//! Common functionality.

#![warn(missing_copy_implementations)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A two-dimensional vector.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

/// Alias for vectors, used to better state intent in certain places.
pub type Point = Vector;

/// An axis-aligned rectangle.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub position: Point,
    pub size: Vector,
}

/// An 8-bit RGBA color.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Vector {
    /// Creates a new vector from X/Y coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns the length² of a vector.
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length of a vector.
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Returns the distance between this point and another point.
    pub fn distance(self, other: Vector) -> f32 {
        (other - self).length()
    }
}

/// Shorthand for `Vector::new(x, y)`.
pub fn vector(x: f32, y: f32) -> Vector {
    Vector::new(x, y)
}

/// Shorthand for `Point::new(x, y)`, which is equivalent to `Vector::new(x, y)`.
pub fn point(x: f32, y: f32) -> Point {
    Point::new(x, y)
}

impl Color {
    /// Solid black.
    pub const BLACK: Self = rgb(0, 0, 0);
    /// Solid white.
    pub const WHITE: Self = rgb(255, 255, 255);
    /// Fully transparent black.
    pub const TRANSPARENT: Self = rgba(0, 0, 0, 0);

    /// Creates a new color from channels.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a new color from an ARGB hex literal.
    pub const fn argb(hex: u32) -> Self {
        let a = (hex >> 24) & 0xFF;
        let r = (hex >> 16) & 0xFF;
        let g = (hex >> 8) & 0xFF;
        let b = hex & 0xFF;
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: a as u8,
        }
    }

    /// Converts a color to a u32 holding ARGB in its bits, from most significant to least significant.
    pub const fn to_argb(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Creates a new color from an RGB hex literal.
    pub const fn rgb(hex: u32) -> Self {
        Self::argb(hex | 0xFF000000)
    }

    /// Returns a color with the same RGB channels, but with the alpha channel altered.
    pub const fn with_alpha(self, a: u8) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
}

/// Creates a new color from RGB channels.
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::new(r, g, b, 255)
}

/// Creates a new color from RGBA channels. Shorthand for `Color::new(r, g, b, a)`.
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(r, g, b, a)
}

impl Rect {
    /// Creates a new rectangle from a position and a size.
    pub fn new(position: impl Into<Point>, size: impl Into<Vector>) -> Self {
        Self {
            position: position.into(),
            size: size.into(),
        }
    }

    /// Returns the X coordinate of the rectangle's position.
    pub fn x(&self) -> f32 {
        self.position.x
    }

    /// Returns the Y coordinate of the rectangle's position.
    pub fn y(&self) -> f32 {
        self.position.y
    }

    /// Returns the width of the rectangle.
    pub fn width(&self) -> f32 {
        self.size.x
    }

    /// Returns the width of the rectangle.
    pub fn height(&self) -> f32 {
        self.size.y
    }

    /// Returns the left side of the rectangle.
    pub fn left(&self) -> f32 {
        self.position.x
    }

    /// Returns the top side of the rectangle.
    pub fn top(&self) -> f32 {
        self.position.y
    }

    /// Returns the right side of the rectangle.
    pub fn right(&self) -> f32 {
        self.position.x + self.size.x
    }

    /// Returns the bottom side of the rectangle.
    pub fn bottom(&self) -> f32 {
        self.position.y + self.size.y
    }

    /// Returns the top left corner of the rectangle.
    pub fn top_left(&self) -> Point {
        self.position
    }

    /// Returns the top right corner of the rectangle.
    pub fn top_right(&self) -> Point {
        self.position + vector(self.size.x, 0.0)
    }

    /// Returns the bottom left corner of the rectangle.
    pub fn bottom_left(&self) -> Point {
        self.position + vector(0.0, self.size.y)
    }

    /// Returns the bottom right corner of the rectangle.
    pub fn bottom_right(&self) -> Point {
        self.position + self.size
    }

    /// Returns the center point of the rectangle.
    pub fn center(&self) -> Point {
        self.position + self.size / 2.0
    }

    /// Returns the horizontal (X) center of the rectangle.
    pub fn center_x(&self) -> f32 {
        self.position.x + self.size.x / 2.0
    }

    /// Returns the vertical (Y) center of the rectangle.
    pub fn center_y(&self) -> f32 {
        self.position.y + self.size.y / 2.0
    }

    /// Returns a _sorted_ rectangle - that is, the same rectangle, but with the width and height
    /// guaranteed to be positive.
    pub fn sort(self) -> Self {
        let left = f32::min(self.left(), self.right());
        let right = f32::max(self.left(), self.right());
        let top = f32::min(self.top(), self.bottom());
        let bottom = f32::max(self.top(), self.bottom());
        Self::new(point(left, top), vector(right - left, bottom - top))
    }
}

impl Default for Vector {
    /// The default vector is `[0.0, 0.0]`.
    fn default() -> Self {
        vector(0.0, 0.0)
    }
}

impl From<(f32, f32)> for Vector {
    fn from(tuple: (f32, f32)) -> Self {
        vector(tuple.0, tuple.1)
    }
}

impl From<[f32; 2]> for Vector {
    fn from(array: [f32; 2]) -> Self {
        vector(array[0], array[1])
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vector(-self.x, -self.y)
    }
}

impl<T: Into<Self>> Add<T> for Vector {
    type Output = Self;

    fn add(self, v: T) -> Self {
        let v = v.into();
        vector(self.x + v.x, self.y + v.y)
    }
}

impl<T: Into<Self>> AddAssign<T> for Vector {
    fn add_assign(&mut self, v: T) {
        let v = v.into();
        self.x += v.x;
        self.y += v.y;
    }
}

impl<T: Into<Self>> Sub<T> for Vector {
    type Output = Self;

    fn sub(self, v: T) -> Self {
        let v = v.into();
        vector(self.x - v.x, self.y - v.y)
    }
}

impl<T: Into<Self>> SubAssign<T> for Vector {
    fn sub_assign(&mut self, v: T) {
        let v = v.into();
        self.x -= v.x;
        self.y -= v.y;
    }
}

impl<T: Into<Self>> Mul<T> for Vector {
    type Output = Self;

    fn mul(self, v: T) -> Self {
        let v = v.into();
        vector(self.x * v.x, self.y * v.y)
    }
}

impl<T: Into<Self>> MulAssign<T> for Vector {
    fn mul_assign(&mut self, v: T) {
        let v = v.into();
        self.x *= v.x;
        self.y *= v.y;
    }
}

impl<T: Into<Self>> Div<T> for Vector {
    type Output = Self;

    fn div(self, v: T) -> Self {
        let v = v.into();
        vector(self.x / v.x, self.y / v.y)
    }
}

impl<T: Into<Self>> DivAssign<T> for Vector {
    fn div_assign(&mut self, v: T) {
        let v = v.into();
        self.x /= v.x;
        self.y /= v.y;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        vector(self.x * s, self.y * s)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, s: f32) {
        self.x *= s;
        self.y *= s;
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        vector(self.x / s, self.y / s)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, s: f32) {
        self.x /= s;
        self.y /= s;
    }
}
