use rendering::Material;
use tracy::{
    canvas::Color,
    math::{Point, Vector},
    query::World,
    rendering::{self, PointLight},
};
pub use utils::*;

mod utils;

#[test]
fn lighting_with_the_surface_in_shadow() {
    let eye = Vector::from_vector(0.0, 0.0, -1.0);
    let normal = Vector::from_vector(0.0, 0.0, -1.0);
    let light = PointLight {
        position: Point::from_point(0.0, 0.0, -10.0),
        color: Color::new(1.0, 1.0, 1.0),
        intensity: 1.0,
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
