use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    math::Matrix,
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
    shape::Plane,
};

use super::Scene;

/// A rendering of the final scene from Chapter 9.
#[derive(Debug)]
pub struct PlaneShape {
    world: World,
    camera: Camera,

    default_transform: Matrix,
    plane_y: f32,
}

impl PlaneShape {
    pub fn new() -> Result<Self> {
        let (world, camera) =
            serde_yaml::from_reader::<_, ScenePrefab>(File::open("scenes/ch09.yml")?)?.build();

        let default_transform = world
            .objects()
            .filter(|&obj| obj.shape().as_any().is::<Plane>())
            .map(|obj| obj.transform().clone())
            .next()
            .unwrap();

        Ok(Self {
            world,
            camera,
            default_transform,
            plane_y: 0.0,
        })
    }
}

impl Scene for PlaneShape {
    fn name(&self) -> String {
        "Chapter 9: Planes".to_string()
    }

    fn description(&self) -> String {
        "Three little spheres sitting on a plane.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        for obj in self.world.objects_mut() {
            if obj.shape().as_any().is::<Plane>() {
                obj.set_transform(
                    Matrix::from_translation(0.0, self.plane_y, 0.0) * &self.default_transform,
                );
                break;
            }
        }

        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        ui.slider(
            &format!("Plane Y##{}", self.name()),
            -10.0,
            10.0,
            &mut self.plane_y,
        )
    }
}
