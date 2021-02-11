//! Basic elements of ray tracing computations.

use std::{cmp::Ordering, vec::IntoIter};

use crate::math::{Coords, MatrixN};

/// Trait of objects which can be tested for intersection with a ray.
pub trait RayCast {
    /// Computes all the intersection points between `self` and `ray`, using transform `m`.
    fn toi_and_normal_with_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections>;

    /// Computes all the TOIs between `self` and `ray` using transform `m`.
    fn toi_with_ray(&self, m: &MatrixN, ray: &Ray) -> Vec<f32> {
        self.toi_and_normal_with_ray(m, ray)
            .map(|xs| xs.map(|x| x.toi).collect())
            .unwrap_or_default()
    }

    /// Returns true if this ray hits `self`.
    fn intersects_ray(&self, m: &MatrixN, ray: &Ray) -> bool {
        self.toi_and_normal_with_ray(m, ray).is_some()
    }
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

    /// Creates a new ray by applying a transformation to `self`.
    pub fn transform_by(&self, m: &MatrixN) -> Self {
        Self {
            origin: m * self.origin,
            dir: m * self.dir,
        }
    }

    /// Computes the position of this ray after walking for `t` times from its starting point
    /// along its direction.
    pub fn point_at(&self, t: f32) -> Coords {
        self.origin + self.dir * t
    }
}

/// Properties of an intersection between a [`Ray`] and a [`Shape`].
#[derive(Debug, Clone)]
pub struct RayIntersection {
    /// The time of impact of this intersection.
    pub toi: f32,
    /// The normal vector at the point of impact.
    pub normal: Coords,
}

impl RayIntersection {
    /// Creates a new intersection.
    pub fn new(toi: f32, normal: Coords) -> Self {
        Self { toi, normal }
    }
}

/// Iterator over all the intersections between a [`Ray`] and a [`Shape`].
#[derive(Debug, Clone)]
pub struct RayIntersections {
    pub(crate) intersections: IntoIter<RayIntersection>,
}

impl From<IntoIter<RayIntersection>> for RayIntersections {
    fn from(intersections: IntoIter<RayIntersection>) -> Self {
        Self { intersections }
    }
}

impl Iterator for RayIntersections {
    type Item = RayIntersection;

    fn next(&mut self) -> Option<Self::Item> {
        self.intersections.next()
    }
}

impl RayIntersections {
    /// Returns the first intersection to have hit the target.
    pub fn hit(self) -> Option<RayIntersection> {
        self.filter(|r| r.toi >= 0.)
            .min_by(|a, b| a.toi.partial_cmp(&b.toi).unwrap_or(Ordering::Greater))
    }
}