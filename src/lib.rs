//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod canvas;
pub mod math;
pub mod query;
pub mod scene;
pub mod shape;

use scene::SceneSize;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

/// Performs the required initialization for this crate.
///
/// **Important:** this function should be called once by the JavaScript code, when the page is
/// first loaded and before any other function. Failing to do so might result in things not working
/// properly.
#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Returns the desired canvas size to render scene `id`.
///
/// `UNDEFINED` is returned if `id` is not a valid scene identifier.
#[wasm_bindgen(js_name = getCanvasSize)]
pub fn get_canvas_size(id: String) -> Result<SceneSize, JsValue> {
    Ok(scene::get_config(id).ok_or(JsValue::UNDEFINED)?.size)
}

/// Renders scene `id` to `ctx`.
///
/// `UNDEFINED` is returned if `id` is not a valid scene identifier.
#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, id: String) -> Result<(), JsValue> {
    let config = scene::get_config(id).ok_or(JsValue::UNDEFINED)?;

    let scene = (config.render_fn)(config.size.width, config.size.height);
    let pixels = scene
        .iter()
        .flat_map(|c| {
            let (r, g, b) = c.to_rgb888();
            vec![r, g, b, 255]
        })
        .collect::<Vec<_>>();

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        config.size.width as u32,
        config.size.height as u32,
    )?;

    ctx.put_image_data(&data, 0.0, 0.0)
}
