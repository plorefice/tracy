use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 7.
#[derive(Debug)]
pub struct ThreeSpheres {
    world: World,
    camera: Camera,
    fov: f32,
}

impl ThreeSpheres {
    pub fn new() -> Result<Self> {
        let (world, camera) =
            serde_yaml::from_reader::<_, ScenePrefab>(File::open("scenes/ch07.yml")?)?.build();

        Ok(Self {
            world,
            camera,
            fov: 60.0,
        })
    }
}

impl Scene for ThreeSpheres {
    fn name(&self) -> String {
        "Chapter 7: Making a Scene".to_string()
    }

    fn description(&self) -> String {
        "Camera pointed at three spheres in a room.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        self.camera.set_size(width, height);
        self.camera.set_fov(self.fov.to_radians());
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        Slider::new(&im_str!("FOV##{}", self.name()))
            .range(30.0..=180.0)
            .build(ui, &mut self.fov)
    }
}
