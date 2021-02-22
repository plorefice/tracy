use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the cover image in appendix A1.
#[derive(Debug)]
pub struct Cover {
    world: World,
    camera: Camera,
}

impl Cover {
    pub fn new() -> Result<Self> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/cover.yml")?)?;

        let mut world = World::new();
        for light in scene.lights.into_iter() {
            world.add_light(light);
        }
        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        Ok(Self {
            world,
            camera: scene.camera.build(),
        })
    }
}

impl Scene for Cover {
    fn name(&self) -> String {
        "Appendix A1: Rendering the Cover Image".to_string()
    }

    fn description(&self) -> String {
        "Looks weird, but ok.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
