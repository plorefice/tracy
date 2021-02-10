//! Geometric queries for ray tracing.

use std::{cmp::Ordering, slice::Iter, vec::IntoIter};

use crate::{
    math::{Coords, MatrixN},
    shape::{Shape, ShapeHandle},
};

/// Properties of an intersection between a [`Ray`] and a [`Shape`].
#[derive(Debug, Clone)]
pub struct RayIntersection {
    /// The time of impact of this intersection.
    pub toi: f32,
}

impl RayIntersection {
    /// Creates a new intersection.
    pub fn new(toi: f32) -> Self {
        Self { toi }
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

/// Trait of objects which can be tested for intersection with a ray.
pub trait RayCast {
    /// Computes all the intersection poinst between `self` and `ray`.
    fn intersects_ray(&self, ray: &Ray) -> Option<RayIntersections>;
}

impl RayCast for dyn Shape {
    fn intersects_ray(&self, ray: &Ray) -> Option<RayIntersections> {
        self.as_ray_cast()
            .expect("this shape does not implement `RayCast`")
            .intersects_ray(ray)
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

/// A handle to an object in a world.
#[derive(Debug, Clone, Copy)]
pub struct WorldHandle(u32);

/// A container of shapes.
#[derive(Debug, Default)]
pub struct World {
    shapes: Vec<ShapeHandle>,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Adds a shape to this world.
    pub fn add(&mut self, shape: ShapeHandle) -> WorldHandle {
        self.shapes.push(shape);
        WorldHandle(self.shapes.len() as u32 - 1)
    }

    /// Returns a reference to the shape identified by this handle.
    pub fn get(&self, handle: WorldHandle) -> Option<&ShapeHandle> {
        self.shapes.get(handle.0 as usize)
    }

    /// Computes the intersections between all the shapes in this world and a ray.
    pub fn interferences_with_ray<'a>(&'a self, ray: &'a Ray) -> InterferencesWithRay<'a> {
        InterferencesWithRay {
            ray,
            handles: self.shapes.iter(),
        }
    }
}

/// Iterator over all the shapes in the world that intersect a specific ray.
#[derive(Debug, Clone)]
pub struct InterferencesWithRay<'a> {
    ray: &'a Ray,
    handles: Iter<'a, ShapeHandle>,
}

impl<'a> Iterator for InterferencesWithRay<'a> {
    type Item = (&'a ShapeHandle, RayIntersections);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(handle) = self.handles.next() {
            if let Some(intersections) = handle.intersects_ray(self.ray) {
                return Some((handle, intersections));
            }
        }

        None
    }
}
