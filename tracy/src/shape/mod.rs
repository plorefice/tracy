//! Collision shapes supported by the ray tracer.

mod sphere;

use std::{fmt::Debug, ops::Deref, sync::Arc};

pub use sphere::*;

use crate::query::RayCast;

/// Traits common to all shapes.
pub trait Shape: Debug + RayCast {}

/// A shareable handle to an abstract shape.
#[derive(Debug, Clone)]
pub struct ShapeHandle(Arc<dyn Shape>);

impl ShapeHandle {
    /// Creates a shareable handle from a shape.
    #[inline]
    pub fn new<S: 'static + Shape>(shape: S) -> Self {
        Self(Arc::new(shape))
    }
}

impl AsRef<dyn Shape> for ShapeHandle {
    #[inline]
    fn as_ref(&self) -> &(dyn Shape + 'static) {
        &*self.0
    }
}

impl Deref for ShapeHandle {
    type Target = dyn Shape;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
