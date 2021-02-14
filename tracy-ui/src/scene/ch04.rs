use std::f32;

use imgui::*;
use tracy::{
    canvas::{Canvas, Color},
    math::{MatrixN, Point},
};

use super::Scene;

/// A rendering of the final scene from Chapter 4.
pub struct Clock;

impl Scene for Clock {
    fn name(&self) -> String {
        "Chapter 4: Matrix Transformations".to_string()
    }

    fn description(&self) -> String {
        "12-hour analog clock built using matrix transformations.".to_string()
    }

    fn render(&self, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height);

        let (wf, hf) = (width as f32, height as f32);

        let radius = wf / 3.;
        let move_to_center = MatrixN::from_translation(wf / 2., hf / 2., 0.);

        for i in 0..12 {
            let rotate = MatrixN::from_rotation_z(f32::consts::PI / 6. * i as f32);
            let pos = &move_to_center * rotate * Point::from_point(0., radius, 0.);

            canvas.put(pos.x as usize, pos.y as usize, Color::new(1., 1., 1.));
        }

        canvas
    }

    fn draw(&mut self, _: &Ui) {}
}
