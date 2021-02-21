use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Camera, ScenePrefab, Stream},
};

use super::Scene;

/// A rendering of the final scene from Chapter 10.
#[derive(Debug)]
pub struct Patterns {
    world: World,
    camera: Camera,
    selection: usize,
}

impl Patterns {
    const SUBSCENES: [(&'static str, &'static str); 3] = [
        ("Basic patterns", "scenes/ch10a.yml"),
        ("Nested patterns", "scenes/ch10b.yml"),
        ("Blended patterns", "scenes/ch10c.yml"),
    ];

    pub fn new() -> Result<Self> {
        Self::load_scene(0)
    }

    fn load_scene(i: usize) -> Result<Self> {
        let scene: ScenePrefab = serde_yaml::from_reader(File::open(Self::SUBSCENES[i].1)?)?;

        let mut world = World::new();
        world.set_light(scene.light);
        for obj in scene.objects.into_iter() {
            world.add(obj);
        }

        Ok(Self {
            world,
            camera: scene.camera.build(),
            selection: i,
        })
    }
}

impl Scene for Patterns {
    fn name(&self) -> String {
        "Chapter 10: Patterns".to_string()
    }

    fn description(&self) -> String {
        "All four patterns in a scene.".to_string()
    }

    fn render(&mut self, width: u32, height: u32) -> Stream {
        self.camera.set_size(width, height);
        self.camera.stream(&self.world)
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        let mut redraw = false;

        if let Some(token) = ComboBox::new(&im_str!("Scene selector##{}", self.name()))
            .preview_value(&ImString::new(Self::SUBSCENES[self.selection].0))
            .begin(ui)
        {
            for (i, &(name, _)) in Self::SUBSCENES.iter().enumerate() {
                if Selectable::new(&ImString::new(name)).build(ui) {
                    *self = Self::load_scene(i).unwrap();
                    redraw = true;
                }
            }
            token.end(ui);
        }

        redraw
    }
}
