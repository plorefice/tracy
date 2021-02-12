//! The unit sphere shape.

use crate::{
    math::{MatrixN, Point},
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
    fn toi_and_normal_with_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections> {
        let inv = m.inverse()?;
        let ray = ray.transform_by(&inv);
        let distance = ray.origin - Point::from_point(0., 0., 0.);

        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&distance);
        let c = distance.dot(&distance) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            None
        } else {
            Some(RayIntersections::from(
                [
                    (-b - discriminant.sqrt()) / (2. * a),
                    (-b + discriminant.sqrt()) / (2. * a),
                ]
                .iter()
                .map(|&toi| {
                    let normal = inv.transpose() * (ray.origin + ray.dir * toi);
                    RayIntersection::new(toi, normal.to_vector().normalize())
                })
                .collect::<Vec<_>>()
                .into_iter(),
            ))
        }
    }
}
