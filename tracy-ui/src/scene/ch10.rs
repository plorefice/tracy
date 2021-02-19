use std::f32::consts::PI;

use imgui::*;
use tracy::{
    math::{MatrixN, Point, Vector},
    query::{Object, World},
    rendering::{Camera, Canvas, Color, Material, Pattern, PointLight},
    shape::{Plane, Sphere},
};

use super::Scene;

/// A rendering of the final scene from Chapter 10.
#[derive(Debug, Clone, Copy)]
pub struct Patterns;

impl Default for Patterns {
    fn default() -> Self {
        Self
    }
}

impl Scene for Patterns {
    fn name(&self) -> String {
        "Chapter 10: Patterns".to_string()
    }

    fn description(&self) -> String {
        "All four patterns in a scene.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut world = World::new();

        // Floor
        world.add(Object::new_with_material(
            Plane,
            MatrixN::identity(4),
            Material {
                pattern: Pattern::Checkers {
                    a: Box::new(Color::new(0.5, 0.5, 0.5).into()),
                    b: Box::new(Color::new(0.2, 0.2, 0.2).into()),
                },
                transform: MatrixN::from_translation(0.0, 0.01, 0.0),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Wall
        world.add(Object::new_with_material(
            Plane,
            MatrixN::from_translation(0.0, 0.0, 2.0) * MatrixN::from_rotation_x(PI / 2.0),
            Material {
                pattern: Pattern::Stripes {
                    a: Box::new(Color::new(0.5, 0.5, 0.5).into()),
                    b: Box::new(Color::new(0.2, 0.2, 0.2).into()),
                },
                transform: MatrixN::from_rotation_y(PI / 4.0),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Left sphere
        world.add(Object::new_with_material(
            Sphere,
            MatrixN::from_translation(-1.0, 1.0, 0.0),
            Material {
                pattern: Pattern::Rings {
                    a: Box::new(Color::new(0.0, 0.8, 0.0).into()),
                    b: Box::new(Color::new(0.0, 0.5, 0.0).into()),
                },
                transform: MatrixN::from_rotation_x(-PI / 4.0)
                    * MatrixN::from_rotation_y(PI / 3.0)
                    * MatrixN::from_scale(0.22, 0.22, 0.22),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Right sphere
        world.add(Object::new_with_material(
            Sphere,
            MatrixN::from_translation(1.0, 0.5, -1.0) * MatrixN::from_scale(0.5, 0.5, 0.5),
            Material {
                pattern: Pattern::LinearGradient {
                    a: Color::new(0.8, 0.0, 0.0),
                    b: Color::new(0.0, 0.8, 0.0),
                },
                transform: MatrixN::from_translation(1.0, 0.0, 0.0)
                    * MatrixN::from_scale(2.0, 2.0, 2.0),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Middle sphere
        world.add(Object::new_with_material(
            Sphere,
            MatrixN::from_translation(0.0, 0.4, -2.0) * MatrixN::from_scale(0.4, 0.4, 0.4),
            Material {
                pattern: Pattern::RadialGradient {
                    a: Color::new(0.0, 0.8, 1.0),
                    b: Color::new(0.0, 0.5, 0.7),
                },
                transform: MatrixN::from_rotation_y(PI / 4.0)
                    * MatrixN::from_rotation_x(-PI / 2.0)
                    * MatrixN::from_scale(0.21, 0.21, 0.21),
                specular: 0.0,
                ..Default::default()
            },
        ));

        world.set_light(PointLight {
            position: Point::from_point(-10.0, 10.0, -10.0),
            ..Default::default()
        });

        let camera = Camera::new_with_transform(
            width,
            height,
            PI / 3.0,
            MatrixN::look_at(
                Point::from_point(0.0, 1.5, -4.0),
                Point::from_point(0.0, 0.5, 0.0),
                Vector::from_vector(0.0, 1.0, 0.0),
            ),
        );

        camera.render(&world)
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
