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
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[derive(Debug, Clone)]
pub struct Cylinder {
    top: f32,
    bottom: f32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            top: f32::INFINITY,
            bottom: f32::NEG_INFINITY,
        }
    }
}

impl Cylinder {
    /// Returns the upper Y coordinate of this cylinder.
    pub fn top(&self) -> f32 {
        self.top
    }

    /// Returns the lower Y coordinate of this cylinder.
    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    /// Changes the upper Y coordinate of `self` to `y`.
    ///
    /// If `y` is lower than the current lower coordinate, it will swap also swap them.
    pub fn set_top(&mut self, y: f32) {
        if y < self.bottom() {
            self.top = self.bottom;
            self.bottom = y;
        } else {
            self.top = y;
        }
    }

    /// Changes the lower Y coordinate of `self` to `y`.
    ///
    /// If `y` is lower than the current lower coordinate, it will swap also swap them.
    pub fn set_bottom(&mut self, y: f32) {
        if y > self.top() {
            self.bottom = self.top;
            self.top = y;
        } else {
            self.bottom = y;
        }
    }
}

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

        if disc < 0.0 {
            return RayIntersections::from(vec![].into_iter());
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let y0 = ray.origin.y + t0 * ray.dir.y;
        let y1 = ray.origin.y + t1 * ray.dir.y;

        let mut xs = Vec::with_capacity(2);

        if self.bottom() < y0 && y0 < self.top() {
            xs.push(RayIntersection {
                toi: t0,
                normal: normal_at(&ray.point_at(t0)),
            });
        }

        if self.bottom() < y1 && y1 < self.top() {
            xs.push(RayIntersection {
                toi: t1,
                normal: normal_at(&ray.point_at(t1)),
            });
        }

        RayIntersections::from(xs.into_iter())
    }
}

fn normal_at(point: &Point3) -> Vec3 {
    Vec3::new(point.x, 0.0, point.z)
}
