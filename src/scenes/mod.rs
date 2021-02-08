//! Generators for each chapter's exercises.

use crate::{
    canvas::{Canvas, Color},
    math::Coords,
};

/// Renders the final scene from Chapter 2.
pub fn chapter_02(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);

    let mut pos = Coords::from_point(0., 1., 0.);
    let mut vel = Coords::from_vector(1., 1.8, 0.).normalize() * 11.25;

    let gravity = Coords::from_vector(0., -0.1, 0.);
    let wind = Coords::from_vector(-0.01, 0., 0.);

    while pos.y > 0. {
        canvas.put(
            pos.x.round() as usize,
            height - pos.y.round() as usize,
            Color::new(1., 1., 1.),
        );

        pos += vel;
        vel += gravity + wind;
    }

    canvas
}
