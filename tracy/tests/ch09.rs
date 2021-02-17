use std::f32::consts::{FRAC_1_SQRT_2, PI};

use tracy::{
    math::{MatrixN, Point, Vector},
    query::Ray,
    rendering::Material,
};
pub use utils::*;

mod utils;

#[test]
fn the_default_transformation() {
    let s = test_shape();
    assert_abs_diff!(s.transform(), MatrixN::identity(4));
}

#[test]
fn assigning_a_transformation() {
    let mut s = test_shape();
    s.set_transform(MatrixN::from_translation(2.0, 3.0, 4.0));
    assert_abs_diff!(s.transform(), MatrixN::from_translation(2.0, 3.0, 4.0));
}

#[test]
fn the_default_material() {
    assert_eq!(test_shape().material(), &Material::default());
}

#[test]
fn assigning_a_material() {
    let mut s = test_shape();
    let m = Material {
        ambient: 1.,
        ..Default::default()
    };

    s.set_material(m);

    assert_eq!(s.material(), &m);
}

#[test]
fn intersecting_a_scaled_shape_with_a_ray() {
    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let mut s = test_shape();
    s.set_transform(MatrixN::from_scale(2.0, 2.0, 2.0));
    s.interferences_with_ray(&r);

    let test_shape = s.shape().as_any().downcast_ref::<TestShape>().unwrap();
    let saved_ray = test_shape.saved_ray.lock().unwrap().unwrap();

    assert_abs_diff!(saved_ray.origin, Point::from_point(0.0, 0.0, -2.5));
    assert_abs_diff!(saved_ray.dir, Vector::from_vector(0.0, 0.0, 0.5));
}

#[test]
fn intersecting_a_translated_shape_with_a_ray() {
    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let mut s = test_shape();
    s.set_transform(MatrixN::from_translation(5.0, 0.0, 0.0));
    s.interferences_with_ray(&r);

    let test_shape = s.shape().as_any().downcast_ref::<TestShape>().unwrap();
    let saved_ray = test_shape.saved_ray.lock().unwrap().unwrap();

    assert_abs_diff!(saved_ray.origin, Point::from_point(-5.0, 0.0, -5.0));
    assert_abs_diff!(saved_ray.dir, Vector::from_vector(0.0, 0.0, 1.0));
}

#[test]
fn computing_the_normal_on_a_translated_shape() {
    let mut s = test_shape();
    s.set_transform(MatrixN::from_translation(0.0, 1.0, 0.0));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    assert_abs_diff!(
        s.interferences_with_ray(&r).unwrap().next().unwrap().normal,
        Vector::from_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
    );
}

#[test]
fn computing_the_normal_on_a_transformed_shape() {
    let mut s = test_shape();
    s.set_transform(MatrixN::from_scale(1.0, 0.5, 1.0) * MatrixN::from_rotation_z(PI / 5.0));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    assert_abs_diff!(
        s.interferences_with_ray(&r).unwrap().next().unwrap().normal,
        Vector::from_vector(0.0, 0.97014, -0.2425)
    );
}
