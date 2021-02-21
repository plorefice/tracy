use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Canvas, ScenePrefab},
};

use super::Scene;

/// A rendering of the final scene from Chapter 11.
#[derive(Debug, Default, Clone, Copy)]
pub struct Reflections;

impl Scene for Reflections {
    fn name(&self) -> String {
        "Chapter 11: Reflection and Refraction".to_string()
    }

    fn description(&self) -> String {
        "Shiny shiny stuff.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Result<Canvas> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch11.yml")?)?;

        let mut world = World::new();

        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        world.set_light(scene.light);

        let mut camera = scene.camera.build();
        camera.set_size(width, height);

        Ok(camera.render(&world))
    }

    fn draw(&mut self, _: &Ui) -> bool {
        false
    }
}
