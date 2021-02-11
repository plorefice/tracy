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
    fn normal_at(&self, p: &Coords) -> Coords {
        (p - Coords::from_point(0., 0., 0.)).normalize()
    }

    #[inline]
    fn as_ray_cast(&self) -> Option<&dyn RayCast> {
        Some(self)
    }
}

impl RayCast for Sphere {
    fn toi_and_normal_with_ray(&self, m: &MatrixN, ray: &Ray) -> Option<RayIntersections> {
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
                    RayIntersection::new(
                        (-b - discriminant.sqrt()) / (2. * a),
                        Coords::from_vector(0., 0., 0.),
                    ),
                    RayIntersection::new(
                        (-b + discriminant.sqrt()) / (2. * a),
                        Coords::from_vector(0., 0., 0.),
                    ),
                ]
                .into_iter(),
            })
        }
    }
}
