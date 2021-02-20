use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    math::Matrix,
    query::World,
    rendering::{Canvas, ScenePrefab},
};

use super::Scene;

/// A rendering of the final scene from Chapter 9.
#[derive(Debug, Clone, Copy)]
pub struct PlaneShape {
    plane_y: f32,
}

impl Default for PlaneShape {
    fn default() -> Self {
        Self { plane_y: 0.0 }
    }
}

impl Scene for PlaneShape {
    fn name(&self) -> String {
        "Chapter 9: Planes".to_string()
    }

    fn description(&self) -> String {
        "Three little spheres sitting on a plane.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Result<Canvas> {
        let mut scene: ScenePrefab = serde_yaml::from_reader(File::open("scenes/ch09.yml")?)?;
        let mut world = World::new();

        scene.objects[0].set_transform(Matrix::from_translation(0.0, self.plane_y, 0.0));

        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        world.set_light(scene.light);

        let mut camera = scene.camera.build();
        camera.set_size(width, height);

        Ok(camera.render(&world))
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        Slider::new(&im_str!("Plane Y##{}", self.name()))
            .range(-10.0..=10.0)
            .build(ui, &mut self.plane_y)
    }
}
