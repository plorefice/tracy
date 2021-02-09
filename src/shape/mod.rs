//! Collision shapes supported by the ray tracer.

use crate::{
    math::Coords,
    query::{Ray, RayCast},
};

/// A sphere.
#[derive(Debug)]
pub struct Sphere;

impl RayCast for Sphere {
    fn intersects_ray(&self, ray: &Ray) -> Vec<f32> {
        let distance = ray.origin - Coords::from_point(0., 0., 0.);

        let a = ray.dir.dot(&ray.dir);
        let b = 2. * ray.dir.dot(&distance);
        let c = distance.dot(&distance) - 1.;

        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2. * a),
                (-b + discriminant.sqrt()) / (2. * a),
            ]
        }
    }
}
