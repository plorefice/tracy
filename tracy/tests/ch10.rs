use tracy::{
    math::Point,
    rendering::{Color, Pattern},
};
pub use utils::*;

mod utils;

#[test]
fn creating_a_stripe_pattern() {
    let _ = Pattern::StripePattern {
        ca: Color::WHITE,
        cb: Color::BLACK,
    };
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Pattern::StripePattern {
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
    let pattern = Pattern::StripePattern {
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
    let pattern = Pattern::StripePattern {
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
