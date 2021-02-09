//! Geometric queries for ray tracing.

use crate::math::Coords;

/// Trait of objects which can be tested for intersection with a ray.
pub trait RayCast {
    /// Computes all the intersection poinst between `self` and `ray`.
    fn intersects_ray(&self, ray: &Ray) -> Vec<f32>;
}

/// A ray starting from a point in space and traveling along a direction.
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    /// Starting point of the ray.
    pub origin: Coords,
    /// Direction of the ray.
    pub dir: Coords,
}

impl Ray {
    /// Creates a ray given its starting point and direction.
    pub fn new(origin: Coords, dir: Coords) -> Self {
        Self {
            origin: Coords::from_point(origin.x, origin.y, origin.z),
            dir: Coords::from_vector(dir.x, dir.y, dir.z),
        }
    }

    /// Computes the position of this ray after walking for `t` times from its starting point
    /// along its direction.
    pub fn point_at(&self, t: f32) -> Coords {
        self.origin + self.dir * t
    }
}
