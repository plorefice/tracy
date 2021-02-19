use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

use tracy::{
    math::{Point, Vector, EPSILON},
    query::{Ray, World},
    rendering::{Color, Material},
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

    assert_eq!(w.reflected_color(&interference), Some(Color::BLACK));
}
