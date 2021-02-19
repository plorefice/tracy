use std::f32::consts::PI;

use imgui::*;
use tracy::{
    math::{Matrix, Point3, Vec3},
    query::{Object, World},
    rendering::{Camera, Canvas, Color, Material, Pattern, PatternKind, PointLight},
    shape::{Plane, Sphere},
};

use super::Scene;

/// A rendering of the final scene from Chapter 10.
#[derive(Debug, Clone, Copy)]
pub struct Patterns {
    selection: usize,
}

impl Default for Patterns {
    fn default() -> Self {
        Self { selection: 0 }
    }
}

impl Patterns {
    const SUBSCENES: [&'static str; 3] = ["Basic patterns", "Nested patterns", "Blended patterns"];

    fn setup_scene(&self, width: u32, height: u32) -> (World, Camera) {
        let mut world = World::new();

        world.set_light(PointLight {
            position: Point3::new(-10.0, 10.0, -10.0),
            ..Default::default()
        });

        let camera = Camera::new_with_transform(
            width,
            height,
            PI / 3.0,
            Matrix::look_at(
                Point3::new(0.0, 1.5, -4.0),
                Point3::new(0.0, 0.5, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        );

        (world, camera)
    }

    fn render_basic(&self, width: u32, height: u32) -> Canvas {
        let (mut world, camera) = self.setup_scene(width, height);

        // Floor
        world.add(Object::new_with_material(
            Plane,
            Matrix::identity(4),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::Checkers {
                        a: Box::new(Pattern::new(Color::new(0.5, 0.5, 0.5).into())),
                        b: Box::new(Pattern::new(Color::new(0.2, 0.2, 0.2).into())),
                    },
                    Matrix::from_translation(0.0, 0.01, 0.0),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Wall
        world.add(Object::new_with_material(
            Plane,
            Matrix::from_translation(0.0, 0.0, 2.0) * Matrix::from_rotation_x(PI / 2.0),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::Stripes {
                        a: Box::new(Pattern::new(Color::new(0.5, 0.5, 0.5).into())),
                        b: Box::new(Pattern::new(Color::new(0.2, 0.2, 0.2).into())),
                    },
                    Matrix::from_rotation_y(PI / 4.0),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Left sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(-1.0, 1.0, 0.0),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::Rings {
                        a: Box::new(Pattern::new(Color::new(0.0, 0.8, 0.0).into())),
                        b: Box::new(Pattern::new(Color::new(0.0, 0.5, 0.0).into())),
                    },
                    Matrix::from_rotation_x(-PI / 4.0)
                        * Matrix::from_rotation_y(PI / 3.0)
                        * Matrix::from_scale(0.22, 0.22, 0.22),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Right sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(1.0, 0.5, -1.0) * Matrix::from_scale(0.5, 0.5, 0.5),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::LinearGradient {
                        a: Color::new(0.8, 0.0, 0.0),
                        b: Color::new(0.0, 0.8, 0.0),
                    },
                    Matrix::from_translation(1.0, 0.0, 0.0) * Matrix::from_scale(2.0, 2.0, 2.0),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        // Middle sphere
        world.add(Object::new_with_material(
            Sphere,
            Matrix::from_translation(0.0, 0.4, -2.0) * Matrix::from_scale(0.4, 0.4, 0.4),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::RadialGradient {
                        a: Color::new(0.0, 0.8, 1.0),
                        b: Color::new(0.0, 0.5, 0.7),
                    },
                    Matrix::from_rotation_y(PI / 4.0)
                        * Matrix::from_rotation_x(-PI / 2.0)
                        * Matrix::from_scale(0.21, 0.21, 0.21),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        camera.render(&world)
    }

    fn render_nested(&self, width: u32, height: u32) -> Canvas {
        let (mut world, camera) = self.setup_scene(width, height);

        let p1 = Pattern::new_with_transform(
            PatternKind::Stripes {
                a: Box::new(Pattern::new(Color::new(0.5, 0.5, 0.5).into())),
                b: Box::new(Pattern::new(Color::new(0.2, 0.2, 0.2).into())),
            },
            Matrix::from_rotation_y(PI / 4.) * Matrix::from_scale(0.25, 0.25, 0.25),
        );

        let p2 = Pattern::new_with_transform(
            PatternKind::Stripes {
                a: Box::new(Pattern::new(Color::new(0.0, 0.3, 0.0).into())),
                b: Box::new(Pattern::new(Color::new(0.0, 0.0, 0.3).into())),
            },
            Matrix::from_rotation_y(-PI / 4.) * Matrix::from_scale(0.25, 0.25, 0.25),
        );

        // Floor
        world.add(Object::new_with_material(
            Plane,
            Matrix::identity(4),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::Checkers {
                        a: Box::new(p1),
                        b: Box::new(p2),
                    },
                    Matrix::from_translation(0.0, 0.01, 0.0),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        camera.render(&world)
    }

    fn render_blended(&self, width: u32, height: u32) -> Canvas {
        let (mut world, camera) = self.setup_scene(width, height);

        let p1 = Pattern::new_with_transform(
            PatternKind::Stripes {
                a: Box::new(Pattern::new(Color::WHITE.into())),
                b: Box::new(Pattern::new(Color::new(0.0, 0.7, 0.0).into())),
            },
            Matrix::from_rotation_y(PI / 4.) * Matrix::from_scale(0.5, 0.5, 0.5),
        );

        let p2 = Pattern::new_with_transform(
            PatternKind::Stripes {
                a: Box::new(Pattern::new(Color::WHITE.into())),
                b: Box::new(Pattern::new(Color::new(0.0, 0.7, 0.0).into())),
            },
            Matrix::from_rotation_y(-PI / 4.) * Matrix::from_scale(0.5, 0.5, 0.5),
        );

        // Floor
        world.add(Object::new_with_material(
            Plane,
            Matrix::identity(4),
            Material {
                pattern: Pattern::new_with_transform(
                    PatternKind::Blended {
                        a: Box::new(p1),
                        b: Box::new(p2),
                    },
                    Matrix::from_translation(0.0, 0.01, 0.0),
                ),
                specular: 0.0,
                ..Default::default()
            },
        ));

        camera.render(&world)
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
        match self.selection {
            0 => self.render_basic(width, height),
            1 => self.render_nested(width, height),
            2 => self.render_blended(width, height),
            _ => unreachable!(),
        }
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        if let Some(token) = ComboBox::new(&im_str!("Scene selector##{}", self.name()))
            .preview_value(&ImString::new(Self::SUBSCENES[self.selection]))
            .begin(ui)
        {
            for (i, &name) in Self::SUBSCENES.iter().enumerate() {
                if Selectable::new(&ImString::new(name)).build(ui) {
                    self.selection = i;
                    redraw = true;
                }
            }
            token.end(ui);
        }

        redraw
    }
}
