//! Geometric queries for ray tracing.

mod collision;
mod ray;

pub use collision::*;
pub use ray::*;

use std::slice::Iter;

/// A handle to an object in a world.
#[derive(Debug, Clone, Copy)]
pub struct CollisionObjectHandle(u32);

/// A container of collidable objects.
#[derive(Debug, Default)]
pub struct World {
    objects: Vec<CollisionObject>,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds an object to this world.
    pub fn add(&mut self, object: CollisionObject) -> CollisionObjectHandle {
        self.objects.push(object);
        CollisionObjectHandle(self.objects.len() as u32 - 1)
    }

    /// Returns a reference to the object identified by this handle.
    pub fn get(&self, handle: CollisionObjectHandle) -> Option<&CollisionObject> {
        self.objects.get(handle.0 as usize)
    }

    /// Returns a mutable reference to the object identified by this handle.
    pub fn get_mut(&mut self, handle: CollisionObjectHandle) -> Option<&mut CollisionObject> {
        self.objects.get_mut(handle.0 as usize)
    }

    /// Computes the intersections between all the object in this world and a ray.
    pub fn interferences_with_ray<'a>(&'a self, ray: &'a Ray) -> InterferencesWithRay<'a> {
        InterferencesWithRay {
            ray,
            objects: self.objects.iter(),
        }
    }
}

/// Iterator over all the objects in the world that intersect a specific ray.
#[derive(Debug, Clone)]
pub struct InterferencesWithRay<'a> {
    ray: &'a Ray,
    objects: Iter<'a, CollisionObject>,
}

impl<'a> Iterator for InterferencesWithRay<'a> {
    type Item = (&'a CollisionObject, RayIntersections);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(obj) = self.objects.next() {
            if let Some(intersections) = obj
                .shape()
                .toi_and_normal_with_ray(obj.transform(), self.ray)
            {
                return Some((obj, intersections));
            }
        }

        None
    }
}
