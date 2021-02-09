//! Generators for each chapter's exercises.

use std::collections::HashMap;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use crate::{
    canvas::{Canvas, Color},
    math::Coords,
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
        map
    };
}

/// Renders the final scene from Chapter 2.
fn chapter_02(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);

    let mut pos = Coords::from_point(0., 1., 0.);
    let mut vel = Coords::from_vector(1., 1.8, 0.).normalize() * 11.25;

    let gravity = Coords::from_vector(0., -0.1, 0.);
    let wind = Coords::from_vector(-0.01, 0., 0.);

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
    Canvas::new(width, height)
}
