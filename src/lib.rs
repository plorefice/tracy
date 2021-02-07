//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

use std::{
    f32::EPSILON,
    ops::{Add, Div, Mul, Neg, Sub},
};

/// A four-dimensional `(x,y,z,w)` tuple  that can represent a point or vector in 3D space.
#[derive(Debug, Default, Clone, Copy)]
pub struct Coords {
    /// The `x` component of this tuple.
    pub x: f32,
    /// The `y` component of this tuple.
    pub y: f32,
    /// The `z` component of this tuple.
    pub z: f32,
    /// The `w` component of this tuple.
    pub w: f32,
}

impl From<(f32, f32, f32, f32)> for Coords {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl Coords {
    /// Creates a new tuple from the coordinates of a point in space.
    pub fn from_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Creates a new tuple from the coordinates of a vector in space.
    pub fn from_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Checks whether this tuple represents a point, ie. its `w` component is equal to 1.
    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() <= EPSILON
    }

    /// Checks whether this tuple represents a vector, ie. its `w` component is equal to 0.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Computes the magnitude of `self`.
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    /// Returns `self` normalized to length 1.0.
    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
            w: self.w / self.length(),
        }
    }

    /// Computes the dot product of `self` and `rhs`.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Computes the cross product of `self` and `rhs`.
    ///
    /// This operation only makes sense on vectors.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::from_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Coords {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Coords {
    type Output = Coords;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
            w: rhs * self.w,
        }
    }
}

impl Div<f32> for Coords {
    type Output = Coords;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}
