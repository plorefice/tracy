use std::f32::consts::PI;

use imgui::*;
use tracy::{
    math::{Matrix, Point3},
    rendering::{Canvas, Color},
};

use super::Scene;

/// A rendering of the final scene from Chapter 4.
#[derive(Debug, Default, Clone, Copy)]
pub struct Clock;

impl Scene for Clock {
    fn name(&self) -> String {
        "Chapter 4: Matrix Transformations".to_string()
    }

    fn description(&self) -> String {
        "12-hour analog clock built using matrix transformations.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut canvas = Canvas::new(width, height);

        let (wf, hf) = (width as f32, height as f32);

        let radius = wf / 3.;
        let move_to_center = Matrix::from_translation(wf / 2., hf / 2., 0.);

        for i in 0..12 {
            let rotate = Matrix::from_rotation_z(PI / 6. * i as f32);
            let pos = &move_to_center * rotate * Point3::new(0., radius, 0.);

            canvas.put(pos.x as u32, pos.y as u32, Color::WHITE);
        }

        canvas
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
