//! The fundamental primitive for ray tracing.

use super::Coords;

/// A line starting from a specific point and traveling along a direction.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Coords,
    direction: Coords,
}

impl Ray {
    /// Creates a ray given its starting point and direction.
    pub fn new(origin: Coords, direction: Coords) -> Self {
        Self {
            origin: Coords::from_point(origin.x, origin.y, origin.z),
            direction: Coords::from_vector(direction.x, direction.y, direction.z),
        }
    }

    /// Returns this ray's starting point.
    pub fn origin(&self) -> Coords {
        self.origin
    }

    /// Returns this ray's direction.
    pub fn direction(&self) -> Coords {
        self.direction
    }

    /// Computes the position of this ray after walking for `t` times from its starting point
    /// along its direction.
    pub fn position(&self, t: f32) -> Coords {
        self.origin() + self.direction() * t
    }
}
