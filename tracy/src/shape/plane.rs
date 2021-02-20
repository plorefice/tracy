use crate::{
    math::{Vec3, EPSILON},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// A plane extending on `xz`.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub struct Plane;

#[cfg_attr(feature = "serde-support", typetag::serde)]
impl Shape for Plane {}

impl RayCast for Plane {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        if ray.dir.y.abs() < EPSILON {
            return RayIntersections::from(Vec::new().into_iter());
        }

        RayIntersections::from(
            vec![RayIntersection {
                toi: -ray.origin.y / ray.dir.y,
                normal: Vec3::unit_y(),
            }]
            .into_iter(),
        )
    }
}
