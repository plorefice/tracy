use std::{f32, fs::File};

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, Color, Material, Pattern, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 5.
#[derive(Debug)]
pub struct FlatSphere {
    world: World,
    camera: Camera,

    color: [f32; 3],
}

impl FlatSphere {
    pub fn new() -> Result<Self> {
        let (world, camera) =
            serde_yaml::from_reader::<_, ScenePrefab>(File::open("scenes/ch05.yml")?)?.build();

        Ok(Self {
            world,
            camera,
            color: [1., 0., 0.],
        })
    }
}

impl Scene for FlatSphere {
    fn name(&self) -> String {
        "Chapter 5: Ray-Sphere Intersections".to_string()
    }

    fn description(&self) -> String {
        "Rendering of a sphere using flat shading.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        let sphere = self.world.objects_mut().next().unwrap();

        sphere.set_material(Material {
            pattern: Pattern::new(Color::from(self.color).into()),
            ..*sphere.material()
        });

        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        ColorPicker::new(&im_str!("Color##{}", self.name()), &mut self.color).build(ui)
    }
}
