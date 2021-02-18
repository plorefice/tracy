use imgui::*;
use tracy::{
    math::{Point, Vector},
    rendering::{Canvas, Color},
};

use super::Scene;

/// A rendering of the final scene from Chapter 2.
#[derive(Debug, Clone, Copy)]
pub struct Trajectory {
    velocity: f32,
}

impl Default for Trajectory {
    fn default() -> Self {
        Self { velocity: 11.25 }
    }
}

impl Scene for Trajectory {
    fn name(&self) -> String {
        "Chapter 2: Drawing on a Canvas".to_string()
    }

    fn description(&self) -> String {
        "Visualization of a projectile's trajectory in 2D space.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut canvas = Canvas::new(width, height);

        let mut pos = Point::from_point(0., 1., 0.);
        let mut vel = Vector::from_vector(1., 1.8, 0.).normalize() * self.velocity;

        let gravity = Vector::from_vector(0., -0.1, 0.);
        let wind = Vector::from_vector(-0.01, 0., 0.);

        while pos.y > 0. {
            if pos.y < height as f32 {
                canvas.put(
                    pos.x.round() as u32,
                    height - pos.y.round() as u32,
                    Color::WHITE,
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
