//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod canvas;
pub mod math;

use canvas::{Canvas, Color};
use math::Coords;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

/// WASM entry point.
#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&render_projectile(width as usize, height as usize)),
        width,
        height,
    )?;

    ctx.put_image_data(&data, 0.0, 0.0)
}

fn render_projectile(width: usize, height: usize) -> Vec<u8> {
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
        .iter()
        .flat_map(|c| {
            let (r, g, b) = c.to_rgb888();
            vec![r, g, b, 255]
        })
        .collect()
}
