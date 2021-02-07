//! Virtual canvas to which the final image will be rendered.

use std::ops::{Add, Mul, Sub};

/// A color in RGB format.
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    /// The red component of this color.
    pub r: f32,
    /// The green component of this color.
    pub g: f32,
    /// The blue component of this color.
    pub b: f32,
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b)
    }
}

impl Color {
    /// Creates a new color from its components.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Returns true if the absolute difference of all components between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.r - other.r).abs() < max_abs_diff
            && (self.g - other.g).abs() < max_abs_diff
            && (self.b - other.b).abs() < max_abs_diff
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

/// A canvas is a rectangular grid of pixels, each with its own [`Color`].
#[derive(Debug, Default, Clone)]
pub struct Canvas {
    grid: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    /// Creates a new canvas with the specified size and all pixels initialized to black.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    /// Returns the width of the canvas.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the canvas.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over the pixes of this canvas.
    ///
    /// The canvas is traversed top-to-bottom, left-to-right.
    pub fn iter(&self) -> std::slice::Iter<Color> {
        self.grid.iter()
    }

    /// Sets the pixel at position `(x,y)` to the specified color.
    ///
    /// # Panics
    ///
    /// Panics if the specified position does not lie within the canvas.
    pub fn put(&mut self, x: usize, y: usize, c: Color) {
        self.grid[y * self.width + x] = c;
    }

    /// Returns the color of the pixel at position `(x,y)`, or `None` is the position is not within
    /// the canvas.
    pub fn get(&self, x: usize, y: usize) -> Option<&Color> {
        self.grid.get(y * self.width + x)
    }
}
