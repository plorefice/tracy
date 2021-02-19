use std::f32::consts::PI;

use imgui::*;
use tracy::{
    math::{Matrix, Point, Vector},
    query::{Object, World},
    rendering::{Camera, Canvas, Color, Material, Pattern, PointLight},
    shape::{Plane, Sphere},
};

use super::Scene;

/// A rendering of the final scene from Chapter 9.
#[derive(Debug, Clone, Copy)]
pub struct PlaneShape {
    plane_y: f32,
}

impl Default for PlaneShape {
    fn default() -> Self {
        Self { plane_y: 0.0 }
    }
}

impl Scene for PlaneShape {
    fn name(&self) -> String {
        "Chapter 9: Planes".to_string()
    }

    fn description(&self) -> String {
        "Three little spheres sitting on a plane.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut world = World::new();

        let floor_mat = Material {
            pattern: Pattern::new(Color::new(1.0, 0.9, 0.9).into()),
            specular: 0.0,
            ..Default::default()
        };

        // Floor
        world.add(Object::new_with_material(
            Plane,
            Matrix::from_translation(0.0, self.plane_y, 0.0),
            floor_mat,
        ));

        // Middle sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(-0.5, 1.0, 0.5),
            Material {
                pattern: Pattern::new(Color::new(0.1, 1.0, 0.5).into()),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            },
        ));

        // Right sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(1.5, 0.5, -0.5) * Matrix::from_scale(0.5, 0.5, 0.5),
            Material {
                pattern: Pattern::new(Color::new(0.5, 1.0, 0.1).into()),
                diffuse: 0.7,
                specular: 0.3,
                ..Default::default()
            },
        ));

        // Left sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(-1.5, 0.33, -0.75) * Matrix::from_scale(0.33, 0.33, 0.33),
            Material {
                pattern: Pattern::new(Color::new(1.0, 0.8, 0.1).into()),
                diffuse: 0.7,
                specular: 0.3,
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
            Matrix::look_at(
                Point::from_point(0.0, 1.5, -5.0),
                Point::from_point(0.0, 1.0, 0.0),
                Vector::from_vector(0.0, 1.0, 0.0),
            ),
        );

        camera.render(&world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        Slider::new(&im_str!("Plane Y##{}", self.name()))
            .range(-10.0..=10.0)
            .build(ui, &mut self.plane_y)
    }
}
