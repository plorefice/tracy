use std::ops::{Add, Mul, Sub};

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
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

    /// The white color.
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

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
