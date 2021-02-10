use crate::{math::MatrixN, shape::ShapeHandle};

/// An object that has a position and shape, and can be hit by rays.
#[derive(Debug, Clone)]
pub struct CollisionObject {
    shape: ShapeHandle,
    transform: MatrixN,
}

impl CollisionObject {
    /// Creates a new object with the given shape and transformation.
    pub fn new(shape: ShapeHandle, transform: MatrixN) -> Self {
        Self { shape, transform }
    }

    /// Returns the shape of this object.
    pub fn shape(&self) -> &ShapeHandle {
        &self.shape
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
