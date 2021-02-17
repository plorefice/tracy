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
