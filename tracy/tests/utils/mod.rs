use tracy::{
    math::MatrixN,
    query::Object,
    shape::{ShapeHandle, Sphere},
};

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
    Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4))
}
