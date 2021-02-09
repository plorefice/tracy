//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod canvas;
pub mod math;
pub mod scenes;

use scenes::SceneSize;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

/// Returns the desired canvas size to render scene `id`.
///
/// `UNDEFINED` is returned if `id` is not a valid scene identifier.
#[wasm_bindgen(js_name = getCanvasSize)]
pub fn get_canvas_size(id: String) -> Result<SceneSize, JsValue> {
    Ok(scenes::get_config(id).ok_or(JsValue::UNDEFINED)?.size)
}

/// Renders scene `id` to `ctx`.
///
/// `UNDEFINED` is returned if `id` is not a valid scene identifier.
#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, id: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let config = scenes::get_config(id).ok_or(JsValue::UNDEFINED)?;

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
