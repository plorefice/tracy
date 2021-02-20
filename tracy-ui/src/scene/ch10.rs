use std::fs::File;

use anyhow::Result;
use imgui::*;
use tracy::{
    query::World,
    rendering::{Canvas, ScenePrefab},
};

use super::Scene;

/// A rendering of the final scene from Chapter 10.
#[derive(Debug, Clone, Copy)]
pub struct Patterns {
    selection: usize,
}

impl Default for Patterns {
    fn default() -> Self {
        Self { selection: 0 }
    }
}

impl Patterns {
    const SUBSCENES: [(&'static str, &'static str); 3] = [
        ("Basic patterns", "scenes/ch10a.yml"),
        ("Nested patterns", "scenes/ch10b.yml"),
        ("Blended patterns", "scenes/ch10c.yml"),
    ];
}

impl Scene for Patterns {
    fn name(&self) -> String {
        "Chapter 10: Patterns".to_string()
    }

    fn description(&self) -> String {
        "All four patterns in a scene.".to_string()
    }

    fn render(&self, width: u32, height: u32) -> Result<Canvas> {
        let scene: ScenePrefab =
            serde_yaml::from_reader(File::open(Self::SUBSCENES[self.selection].1)?)?;

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
        let mut redraw = false;

        if let Some(token) = ComboBox::new(&im_str!("Scene selector##{}", self.name()))
            .preview_value(&ImString::new(Self::SUBSCENES[self.selection].0))
            .begin(ui)
        {
            for (i, &(name, _)) in Self::SUBSCENES.iter().enumerate() {
                if Selectable::new(&ImString::new(name)).build(ui) {
                    self.selection = i;
                    redraw = true;
                }
            }
            token.end(ui);
        }

        redraw
    }
}
