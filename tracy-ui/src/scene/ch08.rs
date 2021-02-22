use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 8.
#[derive(Debug)]
pub struct ShadowSpheres {
    world: World,
    camera: Camera,
    fov: f32,
    cast_shadows: bool,
}

impl ShadowSpheres {
    pub fn new() -> Result<Self> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch08.yml")?)?;

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
            fov: 60.0,
            cast_shadows: true,
        })
    }
}

impl Scene for ShadowSpheres {
    fn name(&self) -> String {
        "Chapter 8: Shadows".to_string()
    }

    fn description(&self) -> String {
        "The three spheres in a room cast shadows now.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        self.world.lights_mut().next().unwrap().casts_shadows = self.cast_shadows;

        self.camera.set_size(width, height);
        self.camera.set_fov(self.fov.to_radians());
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        redraw |= Slider::new(&im_str!("FOV##{}", self.name()))
            .range(30.0..=180.0)
            .build(ui, &mut self.fov);

        redraw |= ui.checkbox(
            &im_str!("Cast shadows##{}", self.name()),
            &mut self.cast_shadows,
        );

        redraw
    }
}
