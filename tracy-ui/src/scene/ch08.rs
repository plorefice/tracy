use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use imgui::*;
use tracy::{
    math::{Matrix, Point3, Vec3},
    query::{Object, World},
    rendering::{Camera, Canvas, Color, Material, Pattern, PointLight},
    shape::Sphere,
};

use super::Scene;

/// A rendering of the final scene from Chapter 8.
#[derive(Debug, Clone, Copy)]
pub struct ShadowSpheres {
    fov: f32,
    cast_shadows: bool,
}

impl Default for ShadowSpheres {
    fn default() -> Self {
        Self {
            fov: 60.0,
            cast_shadows: true,
        }
    }
}

impl Scene for ShadowSpheres {
    fn name(&self) -> String {
        "Chapter 8: Shadows".to_string()
    }

    fn description(&self) -> String {
        "The three spheres in a room cast shadows now.".to_string()
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
            Sphere,
            Matrix::from_scale(10.0, 0.01, 10.0),
            floor_mat.clone(),
        ));

        // Left wall
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(0.0, 0.0, 5.0)
                * Matrix::from_rotation_y(-FRAC_PI_4)
                * Matrix::from_rotation_x(FRAC_PI_2)
                * Matrix::from_scale(10.0, 0.1, 10.0),
            floor_mat.clone(),
        ));

        // Right wall
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(0.0, 0.0, 5.0)
                * Matrix::from_rotation_y(FRAC_PI_4)
                * Matrix::from_rotation_x(FRAC_PI_2)
                * Matrix::from_scale(10.0, 0.1, 10.0),
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
            position: Point3::from_point(-10.0, 10.0, -10.0),
            casts_shadows: self.cast_shadows,
            ..Default::default()
        });

        let camera = Camera::new_with_transform(
            width,
            height,
            self.fov.to_radians(),
            Matrix::look_at(
                Point3::from_point(0.0, 1.5, -5.0),
                Point3::from_point(0.0, 1.0, 0.0),
                Vec3::from_vector(0.0, 1.0, 0.0),
            ),
        );

        camera.render(&world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        redraw |= Slider::new(&im_str!("FOV##{}", self.name()))
            .range(30.0..=180.0)
            .build(ui, &mut self.fov);

        redraw |= ui.checkbox(
            &im_str!("Cast shadows##{}", self.name()),
            &mut self.cast_shadows,
        );

        redraw
    }
}
