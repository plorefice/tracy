//! Generators for each chapter's exercises.

use std::{collections::HashMap, f32};

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use crate::{
    canvas::{Canvas, Color},
    math::{MatrixN, Point, Vector},
    query::{CollisionObject, Ray, World},
    shape::{ShapeHandle, Sphere},
};

/// The dimensions of a scene in pixels.
#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SceneSize {
    /// Horizontal size.
    pub width: usize,
    /// Vertical size.
    pub height: usize,
}

/// Common settings for each rendered scene.
#[derive(Debug)]
pub(crate) struct SceneConfig {
    pub size: SceneSize,
    pub render_fn: Box<fn(usize, usize) -> Canvas>,
}

/// Returns the settings for scene `id`, or `None` if no such scene exists.
pub(crate) fn get_config<S: AsRef<str>>(id: S) -> Option<&'static SceneConfig> {
    SCENES.get(id.as_ref())
}

lazy_static! {
    // TODO: it would be nice to have this in a nice serialization format.
    static ref SCENES: HashMap<&'static str, SceneConfig> = {
        let mut map = HashMap::new();
        map.insert(
            "chapter02",
            SceneConfig {
                size: SceneSize {
                    width: 900,
                    height: 550,
                },
                render_fn: Box::new(chapter_02),
            },
        );
        map.insert(
            "chapter04",
            SceneConfig {
                size: SceneSize {
                    width: 480,
                    height: 480,
                },
                render_fn: Box::new(chapter_04),
            },
        );
        map.insert(
            "chapter05",
            SceneConfig {
                size: SceneSize {
                    width: 512,
                    height: 512,
                },
                render_fn: Box::new(chapter_05),
            },
        );
        map
    };
}

/// Renders the final scene from Chapter 2.
fn chapter_02(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);

    let mut pos = Point::from_point(0., 1., 0.);
    let mut vel = Vector::from_vector(1., 1.8, 0.).normalize() * 11.25;

    let gravity = Vector::from_vector(0., -0.1, 0.);
    let wind = Vector::from_vector(-0.01, 0., 0.);

    while pos.y > 0. {
        canvas.put(
            pos.x.round() as usize,
            height - pos.y.round() as usize,
            Color::new(1., 1., 1.),
        );

        pos += vel;
        vel += gravity + wind;
    }

    canvas
}

/// Renders the final scene from Chapter 4.
fn chapter_04(width: usize, height: usize) -> Canvas {
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

/// Render the final scene from Chapter 5.
fn chapter_05(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut world = World::new();

    let canvas_size = width as f32;

    world.add(CollisionObject::new(
        ShapeHandle::new(Sphere),
        MatrixN::identity(4),
    ));

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

            for (_, xs) in world.interferences_with_ray(&ray) {
                if xs.hit().is_some() {
                    canvas.put(x, y, Color::new(1., 0., 0.));
                }
            }
        }
    }

    canvas
}