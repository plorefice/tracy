use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, PointLight, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 8.
#[derive(Debug)]
pub struct ShadowSpheres {
    world: World,
    camera: Camera,

    fov: f32,
    cast_shadows: bool,
    multiple_lights: bool,
    second_light: PointLight,
}

impl ShadowSpheres {
    pub fn new() -> Result<Self> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch08.yml")?)?;
        let second_light = scene.lights[1].clone();

        let (world, camera) = scene.build();

        Ok(Self {
            world,
            camera,
            fov: 60.0,
            cast_shadows: true,
            multiple_lights: false,
            second_light,
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

        match (self.multiple_lights, self.world.lights().count()) {
            (true, 1) => self.world.add_light(self.second_light.clone()),
            (false, 2) => self.world.remove_light(&self.second_light),
            _ => (),
        }

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

        redraw |= ui.checkbox(
            &im_str!("Multiple ligths##{}", self.name()),
            &mut self.multiple_lights,
        );

        redraw
    }
}
