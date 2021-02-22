use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 12.
#[derive(Debug)]
pub struct Cylinders {
    world: World,
    camera: Camera,
}

impl Cylinders {
    pub fn new() -> Result<Self> {
        let (world, camera) =
            serde_yaml::from_reader::<_, ScenePrefab>(File::open("scenes/ch13.yml")?)?.build();

        Ok(Self { world, camera })
    }
}

impl Scene for Cylinders {
    fn name(&self) -> String {
        "Chapter 13: Cylinders".to_string()
    }

    fn description(&self) -> String {
        "See title.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
