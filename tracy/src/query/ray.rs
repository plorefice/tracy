//! Basic elements of ray tracing computations.

use std::{cmp::Ordering, vec::IntoIter};

use crate::math::{MatrixN, Point, Vector};

/// Trait of objects which can be tested for intersection with a ray.
pub trait RayCast {
    /// Computes all the intersection points between `self` and `ray`, using transform `m`.
    ///
    /// The ray is given in object-space coordinates.
    fn toi_and_normal_with_local_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections>;

    /// Computes all the intersection points between `self` and `ray`, using transform `m`.
    ///
    /// The ray is given in world-space coordinates.
    fn toi_and_normal_with_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections> {
        let local_ray = ray.transform_by(&m.inverse()?);
        self.toi_and_normal_with_local_ray(m, &local_ray)
    }

    /// Computes all the TOIs between `self` and `ray` using transform `m`.
    ///
    /// The ray is given in world-space coordinates.
    fn toi_with_ray(&self, m: &MatrixN, ray: &Ray) -> Vec<f32> {
        self.toi_and_normal_with_ray(m, ray)
            .map(|xs| xs.map(|x| x.toi).collect())
            .unwrap_or_default()
    }

    /// Returns true if this ray hits `self`.
    ///
    /// The ray is given in world-space coordinates.
    fn intersects_ray(&self, m: &MatrixN, ray: &Ray) -> bool {
        self.toi_and_normal_with_ray(m, ray).is_some()
    }
}

/// A ray starting from a point in space and traveling along a direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /// Starting point of the ray.
    pub origin: Point,
    /// Direction of the ray.
    pub dir: Vector,
}

impl Ray {
    /// Creates a ray given its starting point and direction.
    pub fn new(origin: Point, dir: Vector) -> Self {
        Self {
            origin: Point::from_point(origin.x, origin.y, origin.z),
            dir: Vector::from_vector(dir.x, dir.y, dir.z),
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
    pub fn point_at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}

/// Properties of an intersection between a [`Ray`] and a [`Shape`].
#[derive(Debug, Clone)]
pub struct RayIntersection {
    /// The time of impact of this intersection.
    pub toi: f32,
    /// The normal vector at the point of impact.
    pub normal: Vector,
}

impl RayIntersection {
    /// Creates a new intersection.
    pub fn new(toi: f32, normal: Vector) -> Self {
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
