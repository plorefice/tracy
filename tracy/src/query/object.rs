use crate::{math::Matrix, rendering::Material, shape::Shape};

use super::{Ray, RayIntersections};

/// An object that can be positioned in a scene.
#[derive(Debug)]
pub struct Object {
    shape: Box<dyn Shape>,
    material: Material,
    transform: Matrix,
}

impl Object {
    /// Creates a new object with the given shape and transformation.
    pub fn new<S: Shape>(shape: S, transform: Matrix) -> Self {
        Self::new_with_material(shape, transform, Default::default())
    }

    /// Creates a new object with the given material.
    pub fn new_with_material<S: Shape>(shape: S, transform: Matrix, material: Material) -> Self {
        Self {
            shape: Box::new(shape),
            material,
            transform,
        }
    }

    /// Returns the shape of this object.
    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }

    /// Returns a reference to this object's material.
    pub fn material(&self) -> &Material {
        &self.material
    }

    /// Returns a mutable reference to this object's material.
    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    /// Sets this object's material.
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    /// Returns the transform applied to this object's shape.
    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    /// Changes the transform of this object.
    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    /// Computes the intersections between this object and a ray.
    pub fn interferences_with_ray(&self, ray: &Ray) -> RayIntersections {
        self.shape()
            .intersections_in_world_space(self.transform(), ray)
    }
}
