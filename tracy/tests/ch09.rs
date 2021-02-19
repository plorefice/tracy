use std::f32::consts::{FRAC_1_SQRT_2, PI};

use tracy::{
    math::{Matrix, Point3, Vec3},
    query::Ray,
    rendering::Material,
};
pub use utils::*;

mod utils;

#[test]
fn the_default_transformation() {
    let s = test_shape();
    assert_abs_diff!(s.transform(), Matrix::identity(4));
}

#[test]
fn assigning_a_transformation() {
    let mut s = test_shape();
    s.set_transform(Matrix::from_translation(2.0, 3.0, 4.0));
    assert_abs_diff!(s.transform(), Matrix::from_translation(2.0, 3.0, 4.0));
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

    s.set_material(m.clone());

    assert_eq!(s.material(), &m);
}

#[test]
fn intersecting_a_scaled_shape_with_a_ray() {
    let r = Ray::new(
        Point3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    let mut s = test_shape();
    s.set_transform(Matrix::from_scale(2.0, 2.0, 2.0));
    s.interferences_with_ray(&r);

    let test_shape = s.shape().as_any().downcast_ref::<TestShape>().unwrap();
    let saved_ray = test_shape.saved_ray.lock().unwrap().unwrap();

    assert_abs_diff!(saved_ray.origin, Point3::new(0.0, 0.0, -2.5));
    assert_abs_diff!(saved_ray.dir, Vec3::new(0.0, 0.0, 0.5));
}

#[test]
fn intersecting_a_translated_shape_with_a_ray() {
    let r = Ray::new(
        Point3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    let mut s = test_shape();
    s.set_transform(Matrix::from_translation(5.0, 0.0, 0.0));
    s.interferences_with_ray(&r);

    let test_shape = s.shape().as_any().downcast_ref::<TestShape>().unwrap();
    let saved_ray = test_shape.saved_ray.lock().unwrap().unwrap();

    assert_abs_diff!(saved_ray.origin, Point3::new(-5.0, 0.0, -5.0));
    assert_abs_diff!(saved_ray.dir, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn computing_the_normal_on_a_translated_shape() {
    let mut s = test_shape();
    s.set_transform(Matrix::from_translation(0.0, 1.0, 0.0));

    let r = Ray::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    assert_abs_diff!(
        s.interferences_with_ray(&r).next().unwrap().normal,
        Vec3::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
    );
}

#[test]
fn computing_the_normal_on_a_transformed_shape() {
    let mut s = test_shape();
    s.set_transform(Matrix::from_scale(1.0, 0.5, 1.0) * Matrix::from_rotation_z(PI / 5.0));

    let r = Ray::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    assert_abs_diff!(
        s.interferences_with_ray(&r).next().unwrap().normal,
        Vec3::new(0.0, 0.97014, -0.2425)
    );
}

#[test]
fn the_normal_of_a_plane_is_constant_everywhere() {
    let p = plane();

    let origin = Point3::new(0.0, 10.0, 0.0);
    for pt in vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 0.0, -10.0),
        Point3::new(-5.0, 0.0, 150.0),
    ]
    .into_iter()
    {
        let r = Ray::new(origin, pt - origin);

        assert_abs_diff!(
            p.interferences_with_ray(&r).next().unwrap().normal,
            Vec3::new(0.0, 1.0, 0.0)
        );
    }
}

#[test]
fn intersect_with_a_ray_parallel_to_the_plane() {
    let p = plane();
    let r = Ray::new(
        Point3::new(0.0, 10.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    assert_eq!(p.interferences_with_ray(&r).count(), 0);
}

#[test]
fn intersect_with_a_coplanar_ray() {
    let p = plane();
    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(p.interferences_with_ray(&r).count(), 0);
}

#[test]
fn a_ray_intersecting_a_plane_from_above() {
    let p = plane();
    let r = Ray::new(
        Point3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    );

    let xs = p.interferences_with_ray(&r).collect::<Vec<_>>();

    assert_eq!(xs.len(), 1);
    assert_f32!(xs[0].toi, 1.0);
}

#[test]
fn a_ray_intersecting_a_plane_from_below() {
    let p = plane();
    let r = Ray::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let xs = p.interferences_with_ray(&r).collect::<Vec<_>>();

    assert_eq!(xs.len(), 1);
    assert_f32!(xs[0].toi, 1.0);
}
