//! Virtual canvas to which the final image will be rendered.

use std::{
    ops::{Add, Mul, Sub},
    slice,
};

/// A color in RGB format.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
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

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::new(r, g, b)
    }
}

impl Color {
    /// The black color.
    pub const BLACK: Color = Color::new(0., 0., 0.);

    /// Creates a new color from its components.
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Returns true if the absolute difference of all components between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.r - other.r).abs() < max_abs_diff
            && (self.g - other.g).abs() < max_abs_diff
            && (self.b - other.b).abs() < max_abs_diff
    }

    /// Returns the RGB888 representation of `self`.
    pub fn to_rgb888(self) -> (u8, u8, u8) {
        (
            (self.r * 255.).max(0.).min(255.).round() as u8,
            (self.g * 255.).max(0.).min(255.).round() as u8,
            (self.b * 255.).max(0.).min(255.).round() as u8,
        )
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
    width: u32,
    height: u32,
}

impl Canvas {
    /// Creates a new canvas with the specified size and all pixels initialized to black.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    /// Returns the width of the canvas.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the canvas.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns an iterator over the pixels of this canvas.
    ///
    /// The canvas is traversed top-to-bottom, left-to-right.
    pub fn iter(&self) -> slice::Iter<Color> {
        self.grid.iter()
    }

    /// Returns a mutable iterator over the pixels of this canvas.
    ///
    /// The canvas is traversed top-to-bottom, left-to-right.
    pub fn iter_mut(&mut self) -> slice::IterMut<Color> {
        self.grid.iter_mut()
    }

    /// Sets the pixel at position `(x,y)` to the specified color.
    ///
    /// # Panics
    ///
    /// Panics if the specified position does not lie within the canvas.
    pub fn put(&mut self, x: u32, y: u32, c: Color) {
        if x < self.width() && y < self.height() {
            self.grid[(y * self.width + x) as usize] = c;
        }
    }

    /// Returns the color of the pixel at position `(x,y)`, or `None` is the position is not within
    /// the canvas.
    pub fn get(&self, x: u32, y: u32) -> Option<&Color> {
        self.grid.get((y * self.width + x) as usize)
    }

    /// Converts the canvas' contents to PPM format.
    pub fn convert_to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n{}\n", self.width(), self.height(), 255);
        ppm.reserve((self.width() * self.height() * 12) as usize);

        for y in 0..self.height() {
            let mut line_len = 0;

            for x in 0..self.width() {
                let (r, g, b) = self.get(x, y).unwrap().to_rgb888();

                // Lines should not be longer than 70 characters in PPM files.
                // Iterate over each color component in order to split lines as close as possible
                // to the 70 character mark.
                for val in &[r, g, b] {
                    let s = format!("{} ", val);

                    // Swap out the last space for a newline and reset the length counter
                    if line_len + s.len() > 70 {
                        ppm.pop();
                        ppm.push('\n');
                        line_len = 0;
                    }

                    ppm += &s;
                    line_len += s.len();
                }
            }

            ppm.pop();
            ppm.push('\n');
        }

        ppm
    }
}
