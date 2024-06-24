use std::{f32, fs::File};

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, Color, Material, Pattern, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 6.
#[derive(Debug)]
pub struct PhongSphere {
    world: World,
    camera: Camera,

    color: [f32; 3],
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl PhongSphere {
    pub fn new() -> Result<Self> {
        let (world, camera) =
            serde_yaml::from_reader::<_, ScenePrefab>(File::open("scenes/ch06.yml")?)?.build();

        let mat = Material::default();

        Ok(Self {
            world,
            camera,
            color: [1.0, 0.2, 1.0],
            ambient: mat.ambient,
            diffuse: mat.diffuse,
            specular: mat.specular,
            shininess: mat.shininess,
        })
    }
}

impl Scene for PhongSphere {
    fn name(&self) -> String {
        "Chapter 6: Light and Shading".to_string()
    }

    fn description(&self) -> String {
        "Rendering of a sphere using Phong shading.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        let sphere = self.world.objects_mut().next().unwrap();

        sphere.set_material(Material {
            pattern: Pattern::new(Color::from(self.color).into()),
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            shininess: self.shininess,
            ..*sphere.material()
        });

        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        redraw |= ui.slider(
            format!("Ambient##{}", self.name()),
            0.0,
            1.0,
            &mut self.ambient,
        );

        redraw |= ui.slider(
            format!("Diffuse##{}", self.name()),
            0.0,
            1.0,
            &mut self.diffuse,
        );

        redraw |= ui.slider(
            format!("Specular##{}", self.name()),
            0.0,
            1.0,
            &mut self.specular,
        );

        redraw |= ui.slider(
            format!("Shininess##{}", self.name()),
            0.0,
            200.0,
            &mut self.shininess,
        );

        redraw |= ui.color_picker3(format!("Color##{}", self.name()), &mut self.color);

        redraw
    }
}
