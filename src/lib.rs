//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

use std::{
    f32::EPSILON,
    ops::{Add, Neg, Sub},
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

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}
