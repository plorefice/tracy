use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Canvas, ScenePrefab},
};

use super::Scene;

/// A rendering of the final scene from Chapter 7.
#[derive(Debug, Clone, Copy)]
pub struct ThreeSpheres {
    fov: f32,
}

impl Default for ThreeSpheres {
    fn default() -> Self {
        Self { fov: 60.0 }
    }
}

impl Scene for ThreeSpheres {
    fn name(&self) -> String {
        "Chapter 7: Making a Scene".to_string()
    }

    fn description(&self) -> String {
        "Camera pointed at three spheres in a room.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Result<Canvas> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch07.yml")?)?;

        let mut world = World::new();

        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        world.set_light(scene.light);

        let mut camera = scene.camera.build();
        camera.set_size(width, height);
        Ok(camera.render(&world))
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        Slider::new(&im_str!("FOV##{}", self.name()))
            .range(30.0..=180.0)
            .build(ui, &mut self.fov)
    }
}
