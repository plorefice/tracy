use std::f32;

use imgui::*;
use tracy::{
    math::{MatrixN, Point},
    query::{Object, Ray, World},
    rendering::{Canvas, Color},
    shape::Sphere,
};

use super::Scene;

/// A rendering of the final scene from Chapter 5.
#[derive(Debug, Clone, Copy)]
pub struct FlatSphere {
    color: [f32; 3],
}

impl Default for FlatSphere {
    fn default() -> Self {
        Self {
            color: [1., 0., 0.],
        }
    }
}

impl Scene for FlatSphere {
    fn name(&self) -> String {
        "Chapter 5: Ray-Sphere Intersections".to_string()
    }

    fn description(&self) -> String {
        "Rendering of a sphere using flat shading.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        let mut world = World::new();

        let canvas_size = width as f32;

        world.add(Object::new(Sphere, MatrixN::identity(4)));

        let ray_origin = Point::from_point(0., 0., -5.);

        let wall_z = 10.;
        let wall_size = 7.;
        let pixel_size = wall_size / canvas_size;

        for y in 0..height {
            let wall_y = wall_size / 2. - pixel_size * y as f32;

            for x in 0..width {
                let wall_x = -wall_size / 2. + pixel_size * x as f32;

                let target = Point::from_point(wall_x, wall_y, wall_z);
                let ray = Ray::new(ray_origin, (target - ray_origin).normalize());

                if world.interferences_with_ray(&ray).hit().is_some() {
                    canvas.put(x, y, Color::from(self.color));
                }
            }
        }

        canvas
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        ColorPicker::new(&im_str!("Color##{}", self.name()), &mut self.color).build(ui)
    }
}
