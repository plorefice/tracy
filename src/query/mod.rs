//! Geometric queries for ray tracing.

mod ray;

pub use ray::*;

use std::slice::Iter;

use crate::shape::ShapeHandle;

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
