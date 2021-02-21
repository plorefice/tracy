//! The unit cube shape.

use crate::{
    math::{Point3, Vec3},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// The unit cube.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone)]
pub struct Cube;

#[cfg_attr(feature = "serde-support", typetag::serde)]
impl Shape for Cube {}

impl RayCast for Cube {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.dir.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.dir.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.dir.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        RayIntersections::from(
            if tmin > tmax {
                vec![]
            } else {
                vec![
                    RayIntersection {
                        toi: tmin,
                        normal: normal_at(&ray.point_at(tmin)),
                    },
                    RayIntersection {
                        toi: tmax,
                        normal: normal_at(&ray.point_at(tmax)),
                    },
                ]
            }
            .into_iter(),
        )
    }
}

fn check_axis(origin: f32, dir: f32) -> (f32, f32) {
    let tmin = (-1.0 - origin) / dir;
    let tmax = (1.0 - origin) / dir;

    if tmin < tmax {
        (tmin, tmax)
    } else {
        (tmax, tmin)
    }
}

#[allow(clippy::float_cmp)]
fn normal_at(point: &Point3) -> Vec3 {
    let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());

    if maxc == point.x.abs() {
        Vec3::new(point.x, 0.0, 0.0)
    } else if maxc == point.y.abs() {
        Vec3::new(0.0, point.y, 0.0)
    } else {
        Vec3::new(0.0, 0.0, point.z)
    }
}
