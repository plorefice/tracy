use std::f32::consts::FRAC_1_SQRT_2;

use tracy::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{Ray, RayIntersections},
    rendering::{self, Material, PointLight},
};
pub use utils::*;

mod utils;

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point::default(), Vector::from_vector(1., 0., 0.)))
        .and_then(RayIntersections::hit)
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vector::from_vector(1., 0., 0.));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point::default(), Vector::from_vector(0., 1., 0.)))
        .and_then(RayIntersections::hit)
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vector::from_vector(0., 1., 0.));
}

#[test]
fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point::default(), Vector::from_vector(0., 0., 1.)))
        .and_then(RayIntersections::hit)
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vector::from_vector(0., 0., 1.));
}

#[test]
fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    let v = 1. / f32::sqrt(3.);
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point::default(), Vector::from_vector(v, v, v)))
        .and_then(RayIntersections::hit)
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, Vector::from_vector(v, v, v));
}

#[test]
fn the_normal_is_a_normalized_vector() {
    let v = 1. / f32::sqrt(3.);
    let n = sphere()
        .interferences_with_ray(&Ray::new(Point::default(), Vector::from_vector(v, v, v)))
        .and_then(RayIntersections::hit)
        .map(|hit| hit.normal)
        .unwrap();

    assert_abs_diff!(n, n.normalize());
}

#[test]
fn computing_the_normal_on_a_translated_sphere() {
    let mut s = sphere();
    s.set_transform(MatrixN::from_translation(0., 1., 0.));

    let r = Ray::new(
        Point::default(),
        Vector::from_vector(0., 1.70711, -FRAC_1_SQRT_2),
    );

    let mut xs = s.interferences_with_ray(&r).unwrap();

    assert!(xs.any(|x| x.normal.abs_diff_eq(
        &Vector::from_vector(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        1e-4
    )));
}

#[test]
fn computing_the_normal_on_a_transformed_sphere() {
    let mut s = sphere();
    s.set_transform(MatrixN::from_scale(1., 0.5, 1.) * MatrixN::from_rotation_z(0.62832));

    let r = Ray::new(
        Point::default(),
        Vector::from_vector(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
    );

    let mut xs = s.interferences_with_ray(&r).unwrap();

    assert!(xs.any(|x| x
        .normal
        .abs_diff_eq(&Vector::from_vector(0., 0.97014, -0.24254), 1e-4)));
}

#[test]
fn reflecting_a_vector_approaching_at_45_deg() {
    let v = Vector::from_vector(1., -1., 0.);
    let n = Vector::from_vector(0., 1., 0.);
    let r = v.reflect(&n);

    assert_abs_diff!(r, Vector::from_vector(1., 1., 0.));
}

#[test]
fn reflecting_a_vector_off_a_slanted_surface() {
    let v = Vector::from_vector(0., -1., 0.);
    let n = Vector::from_vector(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.);
    let r = v.reflect(&n);

    assert_abs_diff!(r, Vector::from_vector(1., 0., 0.));
}

#[test]
fn a_point_light_has_a_position_and_intensity() {
    let color = Color::new(1., 1., 1.);
    let position = Point::from_point(0., 0., 0.);

    let light = PointLight {
        position,
        color,
        intensity: 1.,
    };

    assert_abs_diff!(light.position, position);
    assert_abs_diff!(light.color, color);
}

#[test]
fn the_default_material() {
    let m = Material::default();

    assert_abs_diff!(m.color, Color::new(1., 1., 1.));
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

    s.set_material(m);

    assert_eq!(s.material(), &m);
}

#[test]
fn lighting_in_several_configurations() {
    for (eyev, normalv, pos, exp) in vec![
        (
            Vector::from_vector(0., 0., -1.),
            Vector::from_vector(0., 0., -1.),
            Point::from_point(0., 0., -10.),
            Color::new(1.9, 1.9, 1.9),
        ),
        (
            Vector::from_vector(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Vector::from_vector(0., 0., -1.),
            Point::from_point(0., 0., -10.),
            Color::new(1., 1., 1.),
        ),
        (
            Vector::from_vector(0., 0., -1.),
            Vector::from_vector(0., 0., -1.),
            Point::from_point(0., 10., -10.),
            Color::new(0.7364, 0.7364, 0.7364),
        ),
        (
            Vector::from_vector(0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            Vector::from_vector(0., 0., -1.),
            Point::from_point(0., 10., -10.),
            Color::new(1.6364, 1.6364, 1.6364),
        ),
        (
            Vector::from_vector(0., 0., -1.),
            Vector::from_vector(0., 0., -1.),
            Point::from_point(0., 0., 10.),
            Color::new(0.1, 0.1, 0.1),
        ),
    ]
    .into_iter()
    {
        let light = PointLight {
            position: pos,
            color: Color::new(1., 1., 1.),
            intensity: 1.,
        };

        let res = rendering::phong_lighting(
            &Material::default(),
            &light,
            &Point::from_point(0., 0., 0.),
            &eyev,
            &normalv,
            false,
        );

        assert_abs_diff!(res, exp);
    }
}
