use crate::{
    math::{Vec3, EPSILON},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// A plane extending on `xz`.
#[derive(Debug, Clone)]
pub struct Plane;

impl Shape for Plane {}

impl RayCast for Plane {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        if ray.dir.y.abs() < EPSILON {
            return RayIntersections::from(Vec::new().into_iter());
        }

        RayIntersections::from(
            vec![RayIntersection {
                toi: -ray.origin.y / ray.dir.y,
                normal: Vec3::from_vector(0.0, 1.0, 0.0),
            }]
            .into_iter(),
        )
    }
}
