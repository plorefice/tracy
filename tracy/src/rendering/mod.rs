//! Rendering primitives and utilities.

mod camera;
mod canvas;
mod color;
mod light;
mod material;
mod pattern;

pub use camera::*;
pub use canvas::*;
pub use color::*;
pub use light::*;
pub use material::*;
pub use pattern::*;

use crate::query::Object;

/// Prefab containing all the elements required to build a renderable scene.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug)]
pub struct ScenePrefab {
	/// The camera in the scene.
    pub camera: CameraPrefab,
	/// The light in the scene.
    pub light: PointLight,
	/// The list of objects in the scene.
    pub objects: Vec<Object>,
}
