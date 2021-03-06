//! The unit sphere shape.

use crate::{
    math::Point3,
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// The unit sphere.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub struct Sphere;

#[cfg_attr(feature = "serde-support", typetag::serde)]
impl Shape for Sphere {}

impl RayCast for Sphere {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        let distance = ray.origin - Point3::new(0.0, 0.0, 0.0);

        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&distance);
        let c = distance.dot(&distance) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return RayIntersections::from(Vec::new().into_iter());
        }

        RayIntersections::from(
            [
                (-b - discriminant.sqrt()) / (2. * a),
                (-b + discriminant.sqrt()) / (2. * a),
            ]
            .iter()
            .map(|&toi| RayIntersection::new(toi, (ray.origin + ray.dir * toi).into()))
            .collect::<Vec<_>>()
            .into_iter(),
        )
    }
}
