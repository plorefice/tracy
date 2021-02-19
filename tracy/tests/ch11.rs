use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

use tracy::{
    math::{MatrixN, Point, Vector, EPSILON},
    query::{Object, Ray, World},
    rendering::{Color, Material, PointLight, DEFAULT_RECURSION_DEPTH},
    shape::Plane,
};
pub use utils::*;

mod utils;

#[test]
fn reflectivity_for_the_default_material() {
    assert_f32!(Material::default().reflective, 0.0);
}

#[test]
fn precomputing_the_reflection_vector() {
    let mut w = World::new();
    w.add(plane());

    let r = Ray::new(
        Point::from_point(0.0, 1.0, -1.0),
        Vector::from_vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        interference.reflect,
        Vector::from_vector(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
    );
}

#[test]
fn the_reflected_color_for_a_nonreflective_material() {
    let mut w = World::default();

    w.objects_mut().nth(1).unwrap().material_mut().ambient = 1.0;

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 1.0).abs() < EPSILON)
        .unwrap();

    assert_eq!(
        w.reflected_color(&interference, DEFAULT_RECURSION_DEPTH),
        Some(Color::BLACK)
    );
}

#[test]
fn the_reflected_color_for_a_reflective_material() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        MatrixN::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -3.0),
        Vector::from_vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.reflected_color(&interference, DEFAULT_RECURSION_DEPTH)
            .unwrap(),
        Color::new(0.19032, 0.2379, 0.14274)
    );
}

#[test]
fn shade_hit_with_a_reflective_material() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        MatrixN::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -3.0),
        Vector::from_vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference, DEFAULT_RECURSION_DEPTH).unwrap(),
        Color::new(0.87677, 0.92436, 0.82918)
    );
}

#[test]
fn color_at_with_mutually_reflective_surfaces() {
    let mut w = World::new();

    w.set_light(PointLight::default());

    w.add(Object::new_with_material(
        Plane,
        MatrixN::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 1.0,
            ..Default::default()
        },
    ));

    w.add(Object::new_with_material(
        Plane,
        MatrixN::from_translation(0.0, 1.0, 0.0),
        Material {
            reflective: 1.0,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, 1.0, 0.0),
    );

    assert!(w.color_at(&r, DEFAULT_RECURSION_DEPTH).is_none());
}

#[test]
fn the_reflected_color_at_the_maximum_recursive_depth() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        MatrixN::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -3.0),
        Vector::from_vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert!(w.reflected_color(&interference, 0).is_none());
}
