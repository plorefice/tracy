use std::f32::EPSILON;

use rendering::Material;
use tracy::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{Object, Ray, World},
    rendering::{self, PointLight},
    shape::{ShapeHandle, Sphere},
};
pub use utils::*;

mod utils;

#[test]
fn lighting_with_the_surface_in_shadow() {
    let eye = Vector::from_vector(0.0, 0.0, -1.0);
    let normal = Vector::from_vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: Point::from_point(0.0, 0.0, -10.0),
        ..Default::default()
    };

    let result = rendering::phong_lighting(
        &Material::default(),
        &light,
        &Point::from_point(0.0, 0.0, 0.0),
        &eye,
        &normal,
        true,
    );

    assert_abs_diff!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
    let w = World::default();
    let p = Point::from_point(0.0, 10.0, 0.0);
    assert!(!w.is_in_shadow(&p));
}

#[test]
fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
    let w = World::default();
    let p = Point::from_point(10.0, -10.0, 10.0);
    assert!(w.is_in_shadow(&p));
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    let w = World::default();
    let p = Point::from_point(-20.0, 20.0, -20.0);
    assert!(!w.is_in_shadow(&p));
}

#[test]
fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    let w = World::default();
    let p = Point::from_point(-2.0, 2.0, -2.0);
    assert!(!w.is_in_shadow(&p));
}

#[test]
fn shade_hit_is_given_an_intersection_in_shadow() {
    let mut w = World::new();

    w.set_light(PointLight {
        position: Point::from_point(0.0, 0.0, -10.0),
        ..Default::default()
    });

    w.add(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));
    w.add(Object::new(
        ShapeHandle::new(Sphere),
        MatrixN::from_translation(0.0, 0.0, 10.0),
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.0).abs() < EPSILON)
        .unwrap();

    let c = w.shade_hit(&interference).unwrap();
    assert_abs_diff!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
fn the_hit_should_offset_the_point() {
    let mut w = World::new();

    w.add(Object::new(
        ShapeHandle::new(Sphere),
        MatrixN::from_translation(0.0, 0.0, 1.0),
    ));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 5.0).abs() < EPSILON)
        .unwrap();

    assert!(interference.over_point.z < -1e-4 / 2.0);
    assert!(interference.point.z > interference.over_point.z);
}
