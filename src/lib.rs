//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod canvas;
pub mod math;
pub mod scenes;

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

/// WASM entry point.
#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let scene = scenes::chapter_02(width as usize, height as usize);
    let pixels = scene
        .iter()
        .flat_map(|c| {
            let (r, g, b) = c.to_rgb888();
            vec![r, g, b, 255]
        })
        .collect::<Vec<_>>();

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels), width, height)?;

    ctx.put_image_data(&data, 0.0, 0.0)
}
