use crate::{math::MatrixN, rendering::Material, shape::ShapeHandle};

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
        Self::new_with_material(shape, Default::default(), transform)
    }

    /// Creates a new object with the given material.
    pub fn new_with_material(shape: ShapeHandle, material: Material, transform: MatrixN) -> Self {
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
}
