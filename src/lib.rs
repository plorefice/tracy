//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod math;

use math::Coords;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

/// WASM entry point.
#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&render_projectile(width as usize, height as usize)),
        width,
        height,
    )?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn render_projectile(width: usize, height: usize) -> Vec<u8> {
    let mut raw_image = Vec::with_capacity(width * height * 4);

    // Fill with black background
    for _ in 0..height {
        for _ in 0..width {
            raw_image.push(0);
            raw_image.push(0);
            raw_image.push(0);
            raw_image.push(255);
        }
    }

    let mut pos = Coords::from_point(0., 1., 0.);
    let mut vel = Coords::from_vector(1., 1., 0.);

    let gravity = Coords::from_vector(0., -0.1, 0.);
    let wind = Coords::from_vector(-0.03, 0., 0.);

    while pos.y > 0. {
        let cx = (pos.x * 15.) as usize;
        let cy = height - (pos.y * 15.) as usize;

        raw_image[cy * 4 * width + cx * 4] = 255;
        raw_image[cy * 4 * width + cx * 4 + 1] = 255;
        raw_image[cy * 4 * width + cx * 4 + 2] = 255;

        pos = pos + vel;
        vel = vel + gravity + wind;
    }

    raw_image
}
