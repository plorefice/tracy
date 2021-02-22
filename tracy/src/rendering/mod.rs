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

use crate::query::{Object, World};

/// Prefab containing all the elements required to build a renderable scene.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug)]
pub struct ScenePrefab {
    /// The camera in the scene.
    pub camera: CameraPrefab,
    /// The lights in the scene.
    pub lights: Vec<PointLight>,
    /// The list of objects in the scene.
    pub objects: Vec<Object>,
}

impl ScenePrefab {
    /// Consumes this prefab and builds the corresponding scene, ie. a world and a camera.
    pub fn build(self) -> (World, Camera) {
        let mut world = World::new();

        for light in self.lights {
            world.add_light(light);
        }

        for obj in self.objects {
            world.add(obj);
        }

        (world, self.camera.build())
    }
}
