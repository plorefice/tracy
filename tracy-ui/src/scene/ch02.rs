use imgui::*;
use tracy::{
    canvas::{Canvas, Color},
    math::{Point, Vector},
};

use super::Scene;

/// A rendering of the final scene from Chapter 2.
pub struct Trajectory {
    pub(super) velocity: f32,
}

impl Scene for Trajectory {
    fn name(&self) -> String {
        "Chapter 2: Drawing on a Canvas".to_string()
    }

    fn description(&self) -> String {
        "Visualization of a projectile's trajectory in 2D space.".to_string()
    }

    fn render(&self, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height);

        let mut pos = Point::from_point(0., 1., 0.);
        let mut vel = Vector::from_vector(1., 1.8, 0.).normalize() * self.velocity;

        let gravity = Vector::from_vector(0., -0.1, 0.);
        let wind = Vector::from_vector(-0.01, 0., 0.);

        while pos.y > 0. {
            if pos.y < height as f32 {
                canvas.put(
                    pos.x.round() as usize,
                    height - pos.y.round() as usize,
                    Color::new(1., 1., 1.),
                );
            }

            pos += vel;
            vel += gravity + wind;
        }

        canvas
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        Slider::new(&im_str!("{}##{}", "Velocity", self.name()))
            .range(0.1..=20.0)
            .build(&ui, &mut self.velocity)
    }
}
