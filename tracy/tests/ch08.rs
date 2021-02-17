use rendering::Material;
use tracy::{
    canvas::Color,
    math::{Point, Vector},
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
