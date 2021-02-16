use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use imgui::*;
use tracy::{
    canvas::{Canvas, Color},
    math::{MatrixN, Point, Vector},
    query::{Object, World},
    rendering::{Camera, Material, PointLight},
    shape::{ShapeHandle, Sphere},
};

use super::Scene;

/// A rendering of the final scene from Chapter 7.
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreeSpheres;

impl Scene for ThreeSpheres {
    fn name(&self) -> String {
        "Chapter 7: Making a Scene".to_string()
    }

    fn description(&self) -> String {
        "Camera pointed at three spheres in a room.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut world = World::new();

        let floor_mat = Material {
            color: Color::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Default::default()
        };

        // Floor
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_scale(10.0, 0.01, 10.0),
            floor_mat,
        ));

        // Left wall
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_translation(0.0, 0.0, 5.0)
                * MatrixN::from_rotation_y(-FRAC_PI_4)
                * MatrixN::from_rotation_x(FRAC_PI_2)
                * MatrixN::from_scale(10.0, 0.01, 10.0),
            floor_mat,
        ));

        // Right wall
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_translation(0.0, 0.0, 5.0)
                * MatrixN::from_rotation_y(FRAC_PI_4)
                * MatrixN::from_rotation_x(FRAC_PI_2)
                * MatrixN::from_scale(10.0, 0.01, 10.0),
            floor_mat,
        ));

        // Middle sphere
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_translation(-0.5, 1.0, 0.5),
            Material {
                color: Color::new(0.1, 1.0, 0.5),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            },
        ));

        // Right sphere
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_translation(1.5, 0.5, -0.5) * MatrixN::from_scale(0.5, 0.5, 0.5),
            Material {
                color: Color::new(0.5, 1.0, 0.1),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            },
        ));

        // Left sphere
        world.add(Object::new_with_material(
            ShapeHandle::new(Sphere),
            MatrixN::from_translation(-1.5, 0.33, -0.75) * MatrixN::from_scale(0.33, 0.33, 0.33),
            Material {
                color: Color::new(1.0, 0.8, 0.1),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            },
        ));

        world.set_light(PointLight {
            position: Point::from_point(-10.0, 10.0, -10.0),
            color: Color::new(1.0, 1.0, 1.0),
            intensity: 1.0,
        });

        let camera = Camera::new_with_transform(
            width,
            height,
            FRAC_PI_3,
            MatrixN::look_at(
                Point::from_point(0.0, 1.5, -5.0),
                Point::from_point(0.0, 1.0, 0.0),
                Vector::from_vector(0.0, 1.0, 0.0),
            ),
        );

        camera.render(&world)
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
