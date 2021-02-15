//! Geometric queries for ray tracing.

mod object;
mod ray;

pub use object::*;
pub use ray::*;

use std::slice::Iter;

use crate::{
    canvas::Color,
    math::{MatrixN, Point},
    rendering::{Material, PointLight},
    shape::{ShapeHandle, Sphere},
};

/// A handle to an object in a world.
#[derive(Debug, Clone, Copy)]
pub struct ObjectHandle(u32);

/// A container of collidable objects.
#[derive(Debug)]
pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let mat = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };

        Self {
            objects: vec![
                Object::new_with_material(ShapeHandle::new(Sphere), MatrixN::identity(4), mat),
                Object::new(ShapeHandle::new(Sphere), MatrixN::from_scale(0.5, 0.5, 0.5)),
            ],
            lights: vec![PointLight {
                position: Point::from_point(-10., 10., -10.),
                color: Color::new(1., 1., 1.),
                intensity: 1.,
            }],
        }
    }
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Adds an object to this world.
    pub fn add(&mut self, object: Object) -> ObjectHandle {
        self.objects.push(object);
        ObjectHandle(self.objects.len() as u32 - 1)
    }

    /// Returns a reference to the object identified by this handle.
    pub fn get(&self, handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(handle.0 as usize)
    }

    /// Returns a mutable reference to the object identified by this handle.
    pub fn get_mut(&mut self, handle: ObjectHandle) -> Option<&mut Object> {
        self.objects.get_mut(handle.0 as usize)
    }

    /// Returns an iterator over this world's objects.
    pub fn objects(&self) -> Iter<Object> {
        self.objects.iter()
    }

    /// Returns an iterator over this world's lights.
    pub fn lights(&self) -> Iter<PointLight> {
        self.lights.iter()
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
    objects: Iter<'a, Object>,
}

impl<'a> Iterator for InterferencesWithRay<'a> {
    type Item = (&'a Object, RayIntersections);

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
