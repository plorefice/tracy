use tracy::{
    math::{MatrixN, Point, Vector},
    query::Object,
    rendering::{self, Color, Material, Pattern, PointLight},
    shape::Sphere,
};
pub use utils::*;

mod utils;

#[test]
fn creating_a_stripe_pattern() {
    let _ = Pattern::Stripes {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Pattern::Stripes {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };

    for y in 0..=2 {
        assert_eq!(
            pattern.color_at(&Point::from_point(0.0, y as f32, 0.0)),
            Color::WHITE
        );
    }
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Pattern::Stripes {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };

    for z in 0..=2 {
        assert_eq!(
            pattern.color_at(&Point::from_point(0.0, 0.0, z as f32)),
            Color::WHITE
        );
    }
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let pattern = Pattern::Stripes {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };

    for (x, exp) in &[
        (0.0, Color::WHITE),
        (0.9, Color::WHITE),
        (1.0, Color::BLACK),
        (-0.1, Color::BLACK),
        (-1.0, Color::BLACK),
        (-1.1, Color::WHITE),
    ] {
        assert_eq!(pattern.color_at(&Point::from_point(*x, 0.0, 0.0)), *exp);
    }
}

#[test]
fn lighting_with_a_pattern_applied() {
    let obj = Object::new_with_material(
        Sphere,
        MatrixN::identity(4),
        Material {
            pattern: Pattern::Stripes {
                ca: Color::WHITE,
                cb: Color::BLACK,
            },
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        },
    );

    let light = PointLight {
        position: Point::from_point(0.0, 0.0, 10.0),
        ..Default::default()
    };

    let eye = Vector::from_vector(0.0, 0.0, -1.0);
    let normal = Vector::from_vector(0.0, 0.0, -1.0);

    let c1 = rendering::phong_lighting(
        &obj,
        &light,
        &Point::from_point(0.9, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );

    let c2 = rendering::phong_lighting(
        &obj,
        &light,
        &Point::from_point(1.1, 0.0, 0.0),
        &eye,
        &normal,
        false,
    );

    assert_eq!(c1, Color::WHITE);
    assert_eq!(c2, Color::BLACK);
}

#[test]
fn stripes_with_an_object_transformation() {
    let obj = Object::new_with_material(
        Sphere,
        MatrixN::from_scale(2.0, 2.0, 2.0),
        Material {
            pattern: Pattern::Stripes {
                ca: Color::WHITE,
                cb: Color::BLACK,
            },
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        },
    );

    let c = rendering::phong_lighting(
        &obj,
        &PointLight::default(),
        &Point::from_point(1.5, 0.0, 0.0),
        &Vector::default(),
        &Vector::default(),
        false,
    );

    assert_abs_diff!(c, Color::WHITE);
}

#[test]
fn stripes_with_a_pattern_transformation() {
    let obj = Object::new_with_material(
        Sphere,
        MatrixN::identity(4),
        Material {
            pattern: Pattern::Stripes {
                ca: Color::WHITE,
                cb: Color::BLACK,
            },
            transform: MatrixN::from_scale(2.0, 2.0, 2.0),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        },
    );

    let c = rendering::phong_lighting(
        &obj,
        &PointLight::default(),
        &Point::from_point(1.5, 0.0, 0.0),
        &Vector::default(),
        &Vector::default(),
        false,
    );

    assert_abs_diff!(c, Color::WHITE);
}

#[test]
fn stripes_with_both_an_object_and_a_pattern_transformation() {
    let obj = Object::new_with_material(
        Sphere,
        MatrixN::from_scale(2.0, 2.0, 2.0),
        Material {
            pattern: Pattern::Stripes {
                ca: Color::WHITE,
                cb: Color::BLACK,
            },
            transform: MatrixN::from_translation(0.5, 0.0, 0.0),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        },
    );

    let c = rendering::phong_lighting(
        &obj,
        &PointLight::default(),
        &Point::from_point(2.5, 0.0, 0.0),
        &Vector::default(),
        &Vector::default(),
        false,
    );

    assert_abs_diff!(c, Color::WHITE);
}

#[test]
fn a_gradient_linearly_interpolates_between_colors() {
    let pattern = Pattern::Gradient {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };

    for (x, exp) in &[
        (0.0, Color::WHITE),
        (0.25, Color::new(0.75, 0.75, 0.75)),
        (0.5, Color::new(0.5, 0.5, 0.5)),
        (0.75, Color::new(0.25, 0.25, 0.25)),
    ] {
        assert_eq!(pattern.color_at(&Point::from_point(*x, 0.0, 0.0)), *exp);
    }
}

#[test]
fn a_ring_should_extend_in_both_x_and_z() {
    let pattern = Pattern::Rings {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };

    for (p, exp) in &[
        (Point::from_point(0.0, 0.0, 0.0), Color::WHITE),
        (Point::from_point(1.0, 0.0, 0.0), Color::BLACK),
        (Point::from_point(0.0, 0.0, 1.0), Color::BLACK),
        (Point::from_point(0.708, 0.0, 0.708), Color::BLACK),
    ] {
        assert_eq!(pattern.color_at(p), *exp);
    }
}
