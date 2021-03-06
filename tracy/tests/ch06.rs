use std::f32::consts::FRAC_1_SQRT_2;

use rendering::Pattern;
use tracy::{
    math::{Matrix, Point3, Vec3, EPSILON},
    query::Ray,
    rendering::{self, Color, Material, PointLight},
};
pub use utils::*;

mod utils;

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point3::default(), Vec3::new(1., 0., 0.)))
        .hit()
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vec3::new(1., 0., 0.));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point3::default(), Vec3::new(0., 1., 0.)))
        .hit()
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vec3::new(0., 1., 0.));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point3::default(), Vec3::new(0., 0., 1.)))
        .hit()
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vec3::new(0., 0., 1.));
}

#[test]
fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    let v = 1. / f32::sqrt(3.);
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point3::default(), Vec3::new(v, v, v)))
        .hit()
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vec3::new(v, v, v));
}

#[test]
fn the_normal_is_a_normalized_vector() {
    let v = 1. / f32::sqrt(3.);
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point3::default(), Vec3::new(v, v, v)))
        .hit()
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, n.normalize());
}

#[test]
fn computing_the_normal_on_a_translated_sphere() {
    let mut s = sphere();
    s.set_transform(Matrix::from_translation(0., 1., 0.));

    let r = Ray::new(Point3::default(), Vec3::new(0., 1.70711, -FRAC_1_SQRT_2));

    let mut xs = s.interferences_with_ray(&r);

    assert!(xs.any(|x| x
        .normal
        .abs_diff_eq(&Vec3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2), EPSILON)));
}

#[test]
fn computing_the_normal_on_a_transformed_sphere() {
    let mut s = sphere();
    s.set_transform(Matrix::from_scale(1., 0.5, 1.) * Matrix::from_rotation_z(0.62832));

    let r = Ray::new(
        Point3::default(),
        Vec3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    let mut xs = s.interferences_with_ray(&r);

    assert!(xs.any(|x| x
        .normal
        .abs_diff_eq(&Vec3::new(0., 0.97014, -0.24254), EPSILON)));
}

#[test]
fn reflecting_a_vector_approaching_at_45_deg() {
    let v = Vec3::new(1., -1., 0.);
    let n = Vec3::new(0., 1., 0.);
    let r = v.reflect(&n);

    assert_abs_diff!(r, Vec3::new(1., 1., 0.));
}

#[test]
fn reflecting_a_vector_off_a_slanted_surface() {
    let v = Vec3::new(0., -1., 0.);
    let n = Vec3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.);
    let r = v.reflect(&n);

    assert_abs_diff!(r, Vec3::new(1., 0., 0.));
}

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let color = Color::WHITE;
    let position = Point3::new(0., 0., 0.);

    let light = PointLight {
        position,
        color,
        ..Default::default()
    };

    assert_abs_diff!(light.position, position);
    assert_abs_diff!(light.color, color);
}

#[test]
fn the_default_material() {
    let m = Material::default();

    assert_eq!(m.pattern, Pattern::new(Color::WHITE.into()));
    assert_f32!(m.ambient, 0.1);
    assert_f32!(m.diffuse, 0.9);
    assert_f32!(m.specular, 0.9);
    assert_f32!(m.shininess, 200.);
}

#[test]
fn a_sphere_has_a_default_material() {
    assert_eq!(sphere().material(), &Material::default());
}

#[test]
fn a_sphere_may_be_assigned_a_material() {
    let mut s = sphere();
    let m = Material {
        ambient: 1.,
        ..Default::default()
    };

    s.set_material(m.clone());

    assert_eq!(s.material(), &m);
}

#[test]
fn lighting_in_several_configurations() {
    for (eyev, normalv, pos, exp) in vec![
        (
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 0., -1.),
            Point3::new(0., 0., -10.),
            Color::new(1.9, 1.9, 1.9),
        ),
        (
            Vec3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Vec3::new(0., 0., -1.),
            Point3::new(0., 0., -10.),
            Color::WHITE,
        ),
        (
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 0., -1.),
            Point3::new(0., 10., -10.),
            Color::new(0.7364, 0.7364, 0.7364),
        ),
        (
            Vec3::new(0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Vec3::new(0., 0., -1.),
            Point3::new(0., 10., -10.),
            Color::new(1.6364, 1.6364, 1.6364),
        ),
        (
            Vec3::new(0., 0., -1.),
            Vec3::new(0., 0., -1.),
            Point3::new(0., 0., 10.),
            Color::new(0.1, 0.1, 0.1),
        ),
    ]
    .into_iter()
    {
        let light = PointLight {
            position: pos,
            ..Default::default()
        };

        let res = rendering::phong_lighting(
            &sphere(),
            &light,
            &Point3::new(0., 0., 0.),
            &eyev,
            &normalv,
            false,
        );

        assert_abs_diff!(res, exp);
    }
}
