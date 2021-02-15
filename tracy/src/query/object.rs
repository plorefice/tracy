use crate::{math::MatrixN, rendering::Material, shape::ShapeHandle};

use super::{Ray, RayCast, RayIntersections};

/// An object that can be positioned in a scene.
#[derive(Debug, Clone)]
pub struct Object {
    shape: ShapeHandle,
    material: Material,
    transform: MatrixN,
}

impl Object {
    /// Creates a new object with the given shape and transformation.
    pub fn new(shape: ShapeHandle, transform: MatrixN) -> Self {
        Self::new_with_material(shape, transform, Default::default())
    }

    /// Creates a new object with the given material.
    pub fn new_with_material(shape: ShapeHandle, transform: MatrixN, material: Material) -> Self {
        Self {
            shape,
            material,
            transform,
        }
    }

    /// Returns the shape of this object.
    pub fn shape(&self) -> &ShapeHandle {
        &self.shape
    }

    /// Returns a reference to this object's material.
    pub fn material(&self) -> &Material {
        &self.material
    }

    /// Sets this object's material.
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    /// Returns the transform applied to this object's shape.
    pub fn transform(&self) -> &MatrixN {
        &self.transform
    }

    /// Changes the transform of this object.
    pub fn set_transform(&mut self, transform: MatrixN) {
        self.transform = transform;
    }

    /// Computes the intersections between this object and a ray.
    pub fn interferences_with_ray(&self, ray: &Ray) -> Option<RayIntersections> {
        self.shape().toi_and_normal_with_ray(self.transform(), ray)
    }
}
