//! Common functionality.

#![warn(missing_copy_implementations)]

use std::ops::{Add, Div, Mul, Sub};

/// A two-dimensional vector.
#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

/// Alias for vectors, used to better state intent in certain places.
pub type Point = Vector;

/// An axis-aligned rectangle.
#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
    pub position: Point,
    pub size: Vector,
}

/// An 8-bit RGBA color.
#[derive(Copy, Clone, PartialEq, Eq)]
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

    /// Creates a new color from channels.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
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
        self.position + point(self.size.x, 0.0)
    }

    /// Returns the bottom left corner of the rectangle.
    pub fn bottom_left(&self) -> Point {
        self.position + point(0.0, self.size.y)
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

impl<T: Into<Self>> Add<T> for Vector {
    type Output = Self;

    fn add(self, v: T) -> Self {
        let v = v.into();
        vector(self.x + v.x, self.y + v.y)
    }
}

impl<T: Into<Self>> Sub<T> for Vector {
    type Output = Self;

    fn sub(self, v: T) -> Self {
        let v = v.into();
        vector(self.x - v.x, self.y - v.y)
    }
}

impl<T: Into<Self>> Mul<T> for Vector {
    type Output = Self;

    fn mul(self, v: T) -> Self {
        let v = v.into();
        vector(self.x * v.x, self.y * v.y)
    }
}

impl<T: Into<Self>> Div<T> for Vector {
    type Output = Self;

    fn div(self, v: T) -> Self {
        let v = v.into();
        vector(self.x / v.x, self.y / v.y)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, s: f32) -> Self {
        vector(self.x * s, self.y * s)
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, s: f32) -> Self {
        vector(self.x / s, self.y / s)
    }
}