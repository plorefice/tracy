use std::sync::Mutex;

use tracy::{
    math::MatrixN,
    query::{Object, Ray, RayCast, RayIntersection, RayIntersections},
    shape::{Shape, Sphere},
};

/// A fake shape to test the [`Shape`] abstractions.
#[derive(Debug)]
pub struct TestShape {
    pub saved_ray: Mutex<Option<Ray>>,
}

impl Shape for TestShape {}

impl RayCast for TestShape {
    fn intersections_in_local_space(&self, ray: &Ray) -> Option<RayIntersections> {
        *self.saved_ray.lock().unwrap() = Some(*ray);

        Some(RayIntersections::from(
            vec![RayIntersection {
                toi: 0.,
                normal: ray.origin + ray.dir,
            }]
            .into_iter(),
        ))
    }
}

#[macro_export]
macro_rules! assert_f32 {
    ($a:expr, $b:expr) => {
        assert!(($a - $b).abs() < 1e-4);
    };
}

#[macro_export]
macro_rules! assert_abs_diff {
    ($a:expr, $b:expr) => {
        assert!(($a).abs_diff_eq(&$b, 1e-4));
    };
}

#[macro_export]
macro_rules! assert_not_abs_diff {
    ($a:expr, $b:expr) => {
        assert!(!($a).abs_diff_eq(&$b, 1e-4));
    };
}

/// Creates a default unit sphere centered in the origin.
pub fn sphere() -> Object {
    Object::new(Sphere, MatrixN::identity(4))
}

/// Creates a test shape centered in the origin.
pub fn test_shape() -> Object {
    Object::new(
        TestShape {
            saved_ray: Mutex::new(None),
        },
        MatrixN::identity(4),
    )
}
