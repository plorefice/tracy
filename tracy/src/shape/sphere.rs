//! The unit sphere shape.

use crate::{
    math::Point,
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// The unit sphere.
#[derive(Debug)]
pub struct Sphere;

impl Shape for Sphere {}

impl RayCast for Sphere {
    fn intersections_in_local_space(&self, ray: &Ray) -> Option<RayIntersections> {
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
                .map(|&toi| RayIntersection::new(toi, ray.origin + ray.dir * toi))
                .collect::<Vec<_>>()
                .into_iter(),
            ))
        }
    }
}
