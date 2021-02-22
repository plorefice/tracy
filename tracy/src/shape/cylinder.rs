//! The unit cylinder shape.

use crate::{
    math::{Point3, Vec3, EPSILON},
    query::{Ray, RayCast, RayIntersection, RayIntersections},
};

use super::Shape;

/// A cylinder with unit radius centered around the origin and extending along the Y axis.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[derive(Debug, Clone)]
pub struct Cylinder {
    top: f32,
    bottom: f32,
    closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            top: f32::INFINITY,
            bottom: f32::NEG_INFINITY,
            closed: false,
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

    /// Returns whether this cylinder is capped at its ends.
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Marks this cylinder's ends as capped if `closed` is true, or uncapped otherwise.
    pub fn set_closed(&mut self, closed: bool) {
        self.closed = closed;
    }

    /// Computes the normal at the given point
    fn normal_at(&self, point: &Point3) -> Vec3 {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.top - EPSILON {
            Vec3::unit_y()
        } else if dist < 1.0 && point.y <= self.bottom + EPSILON {
            -Vec3::unit_y()
        } else {
            Vec3::new(point.x, 0.0, point.z)
        }
    }

    /// Appends to the list of intersections any hits with this cylinder's caps, if capped.
    fn intersections_at_caps(&self, ray: &Ray, xs: &mut Vec<RayIntersection>) {
        if self.closed() && ray.dir.y.abs() > EPSILON {
            for &y in &[self.bottom, self.top] {
                let t = (y - ray.origin.y) / ray.dir.y;
                if check_cap(ray, t) {
                    xs.push(RayIntersection {
                        toi: t,
                        normal: self.normal_at(&ray.point_at(t)),
                    });
                }
            }
        }
    }
}

#[cfg_attr(feature = "serde-support", typetag::serde)]
impl Shape for Cylinder {}

impl RayCast for Cylinder {
    fn intersections_in_local_space(&self, ray: &Ray) -> RayIntersections {
        let mut xs = Vec::with_capacity(2);

        let a = ray.dir.x.powi(2) + ray.dir.z.powi(2);

        if a > EPSILON {
            let b = 2.0 * ray.origin.x * ray.dir.x + 2.0 * ray.origin.z * ray.dir.z;
            let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

            let disc = b.powi(2) - 4.0 * a * c;

            if disc >= 0.0 {
                let t0 = (-b - disc.sqrt()) / (2.0 * a);
                let t1 = (-b + disc.sqrt()) / (2.0 * a);

                let y0 = ray.origin.y + t0 * ray.dir.y;
                let y1 = ray.origin.y + t1 * ray.dir.y;

                if self.bottom() < y0 && y0 < self.top() {
                    xs.push(RayIntersection {
                        toi: t0,
                        normal: self.normal_at(&ray.point_at(t0)),
                    });
                }

                if self.bottom() < y1 && y1 < self.top() {
                    xs.push(RayIntersection {
                        toi: t1,
                        normal: self.normal_at(&ray.point_at(t1)),
                    });
                }
            }
        }

        self.intersections_at_caps(ray, &mut xs);
        RayIntersections::from(xs.into_iter())
    }
}

fn check_cap(ray: &Ray, t: f32) -> bool {
    let x = ray.origin.x + t * ray.dir.x;
    let z = ray.origin.z + t * ray.dir.z;

    (x * x + z * z) <= 1.0
}
