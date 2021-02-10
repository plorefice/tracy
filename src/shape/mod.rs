//! Collision shapes supported by the ray tracer.

use std::{fmt::Debug, ops::Deref, sync::Arc};

use crate::{
    math::Coords,
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

/// Trait implemented by all supported shapes.
pub trait Shape: 'static + Debug + Send + Sync {
    /// The `RayCast` implementation of `self`.
    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast> {
        None
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

/// A sphere.
#[derive(Debug)]
pub struct Sphere;

impl Shape for Sphere {
    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast> {
        Some(self)
    }
}

impl RayCast for Sphere {
    fn intersects_ray(&self, ray: &Ray) -> Option<RayIntersections> {
        let distance = ray.origin - Coords::from_point(0., 0., 0.);

        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&distance);
        let c = distance.dot(&distance) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            Some(RayIntersections {
                intersections: vec![
                    RayIntersection::new((-b - discriminant.sqrt()) / (2. * a)),
                    RayIntersection::new((-b + discriminant.sqrt()) / (2. * a)),
                ]
                .into_iter(),
            })
        }
    }
}
