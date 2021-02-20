use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Canvas, PointLight, ScenePrefab},
};

use super::Scene;

/// A rendering of the final scene from Chapter 8.
#[derive(Debug, Clone, Copy)]
pub struct ShadowSpheres {
    fov: f32,
    cast_shadows: bool,
}

impl Default for ShadowSpheres {
    fn default() -> Self {
        Self {
            fov: 60.0,
            cast_shadows: true,
        }
    }
}

impl Scene for ShadowSpheres {
    fn name(&self) -> String {
        "Chapter 8: Shadows".to_string()
    }

    fn description(&self) -> String {
        "The three spheres in a room cast shadows now.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Result<Canvas> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch08.yml")?)?;

        let mut world = World::new();

        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        world.set_light(PointLight {
            casts_shadows: self.cast_shadows,
            ..scene.light
        });

        let mut camera = scene.camera.build();
        camera.set_size(width, height);
        camera.set_fov(self.fov.to_radians());

        Ok(camera.render(&world))
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
