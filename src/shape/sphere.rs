//! The unit sphere shape.

use crate::{
    math::{Coords, MatrixN},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// The unit sphere.
#[derive(Debug)]
pub struct Sphere;

impl Shape for Sphere {
    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast> {
        Some(self)
    }
}

impl RayCast for Sphere {
    fn intersects_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections> {
        let ray = ray.transform_by(&m.inverse()?);
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
