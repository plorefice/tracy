use std::f32;

use imgui::*;
use tracy::{
    math::{MatrixN, Point},
    query::{Object, Ray, World},
    rendering::{Canvas, Color, Material, Pattern, PointLight},
    shape::Sphere,
};

use super::Scene;

/// A rendering of the final scene from Chapter 6.
#[derive(Debug, Clone, Copy)]
pub struct PhongSphere {
    color: [f32; 3],
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Default for PhongSphere {
    fn default() -> Self {
        let mat = Material::default();
        Self {
            color: [1.0, 0.2, 1.0],
            ambient: mat.ambient,
            diffuse: mat.diffuse,
            specular: mat.specular,
            shininess: mat.shininess,
        }
    }
}

impl Scene for PhongSphere {
    fn name(&self) -> String {
        "Chapter 6: Light and Shading".to_string()
    }

    fn description(&self) -> String {
        "Rendering of a sphere using Phong shading.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Canvas {
        let mut canvas = Canvas::new(width, height);
        let mut world = World::new();

        let canvas_size = width as f32;

        world.add(Object::new_with_material(
            Sphere,
            MatrixN::identity(4),
            Material {
                pattern: Pattern::Solid(Color::new(self.color[0], self.color[1], self.color[2])),
                ambient: self.ambient,
                diffuse: self.diffuse,
                specular: self.specular,
                shininess: self.shininess,
                ..Default::default()
            },
        ));

        world.set_light(PointLight {
            position: Point::from_point(-10., 10., -10.),
            casts_shadows: false,
            ..Default::default()
        });

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

                canvas.put(x, y, world.color_at(&ray).unwrap_or_default());
            }
        }

        canvas
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        redraw |= Slider::new(&im_str!("Ambient##{}", self.name()))
            .range(0.0..=1.0)
            .build(ui, &mut self.ambient);

        redraw |= Slider::new(&im_str!("Diffuse##{}", self.name()))
            .range(0.0..=1.0)
            .build(ui, &mut self.diffuse);

        redraw |= Slider::new(&im_str!("Specular##{}", self.name()))
            .range(0.0..=1.0)
            .build(ui, &mut self.specular);

        redraw |= Slider::new(&im_str!("Shininess##{}", self.name()))
            .range(10.0..=200.0)
            .build(ui, &mut self.shininess);

        redraw |= ColorPicker::new(&im_str!("Color##{}", self.name()), &mut self.color).build(ui);

        redraw
    }
}
