//! The unit cylinder shape.

use crate::{
    math::{Point3, Vec3, EPSILON},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// An infinitely long cylinder with unit radius centered around the origin
/// and extending along the Y axis.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub struct Cylinder;

#[cfg_attr(feature = "serde-support", typetag::serde)]
impl Shape for Cylinder {}

impl RayCast for Cylinder {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        let a = ray.dir.x.powi(2) + ray.dir.z.powi(2);

        if a < EPSILON {
            return RayIntersections::from(vec![].into_iter());
        }

        let b = 2.0 * ray.origin.x * ray.dir.x + 2.0 * ray.origin.z * ray.dir.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;

        RayIntersections::from(
            if disc < 0.0 {
                vec![]
            } else {
                let t0 = (-b - disc.sqrt()) / (2.0 * a);
                let t1 = (-b + disc.sqrt()) / (2.0 * a);

                vec![
                    RayIntersection {
                        toi: t0,
                        normal: normal_at(&ray.point_at(t0)),
                    },
                    RayIntersection {
                        toi: t1,
                        normal: normal_at(&ray.point_at(t1)),
                    },
                ]
            }
            .into_iter(),
        )
    }
}

fn normal_at(point: &Point3) -> Vec3 {
    Vec3::new(point.x, 0.0, point.z)
}
