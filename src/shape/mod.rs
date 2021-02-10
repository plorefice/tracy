//! Collision shapes supported by the ray tracer.

mod sphere;

pub use sphere::*;

use std::{fmt::Debug, ops::Deref, sync::Arc};

use crate::query::{Ray, RayCast, RayIntersections};

/// Trait implemented by all supported shapes.
pub trait Shape: 'static + Debug + Send + Sync {
    /// The `RayCast` implementation of `self`.
    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast> {
        None
    }
}

/// Blanket implementation of [`RayCast`] for a shape.
impl RayCast for dyn Shape {
    fn intersects_ray(&self, ray: &Ray) -> Option<RayIntersections> {
        self.as_ray_cast()
            .expect("this shape does not implement `RayCast`")
            .intersects_ray(ray)
    }
}

/// A shared handle to an abstract shape.
#[derive(Debug, Clone)]
pub struct ShapeHandle(Arc<dyn Shape>);

impl ShapeHandle {
    /// Creates a shareable handle from a shape.
    #[inline]
    pub fn new<S: Shape>(shape: S) -> Self {
        Self(Arc::new(shape))
    }
}

impl AsRef<dyn Shape> for ShapeHandle {
    #[inline]
    fn as_ref(&self) -> &dyn Shape {
        &*self.0
    }
}

impl Deref for ShapeHandle {
    type Target = dyn Shape;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
