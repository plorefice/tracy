//! A Rust implementation of The Ray Tracer Challenge book.

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

pub mod canvas;
pub mod math;
pub mod query;
pub mod rendering;
pub mod scene;
pub mod shape;

use scene::SCENES;
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

/// WASM interface for scene rendering.
#[wasm_bindgen]
#[derive(Debug)]
pub struct SceneRenderer;

#[wasm_bindgen]
impl SceneRenderer {
    /// Renders scene `id` to `ctx`.
    ///
    /// `UNDEFINED` is returned if `id` is not a valid scene identifier.
    #[wasm_bindgen]
    pub fn draw(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        id: String,
        width: usize,
        height: usize,
    ) -> Result<(), JsValue> {
        let scene = (SCENES.get(id.as_str()).ok_or(JsValue::UNDEFINED)?)(width, height);
        let pixels = scene
            .iter()
            .flat_map(|c| {
                let (r, g, b) = c.to_rgb888();
                vec![r, g, b, 255]
            })
            .collect::<Vec<_>>();

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&pixels),
            width as u32,
            height as u32,
        )?;

        ctx.put_image_data(&data, 0.0, 0.0)
    }
}
