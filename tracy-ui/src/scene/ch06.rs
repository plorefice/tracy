use std::f32;

use imgui::*;
use tracy::{
    canvas::{Canvas, Color},
    math::{MatrixN, Point},
    query::{Object, Ray, World},
    rendering::{self, Material, PointLight},
    shape::{ShapeHandle, Sphere},
};

use super::Scene;

/// A rendering of the final scene from Chapter 6.
#[derive(Debug, Default, Clone, Copy)]
pub struct PhongSphere;

impl Scene for PhongSphere {
    fn name(&self) -> String {
        "Chapter 6: Light and Shading".to_string()
    }

    fn description(&self) -> String {
        "Rendering of a sphere using Phong shading.".to_string()
    }

    fn render(&self, width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        let mut world = World::new();

        let canvas_size = width as f32;

        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::identity(4),
            Material {
                color: Color::new(1., 0.2, 1.),
                ..Default::default()
            },
        ));

        let light = PointLight {
            position: Point::from_point(-10., 10., -10.),
            color: Color::new(1., 1., 1.),
            intensity: 1.,
        };

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

                for (obj, xs) in world.interferences_with_ray(&ray) {
                    if let Some(hit) = xs.hit() {
                        let color = rendering::phong_lighting(
                            obj.material(),
                            &light,
                            &ray.point_at(hit.toi),
                            &(-ray.dir),
                            &hit.normal,
                        );

                        canvas.put(x, y, color);
                    }
                }
            }
        }

        canvas
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
