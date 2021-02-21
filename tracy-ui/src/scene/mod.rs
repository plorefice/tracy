//! Generators for each chapter's exercises.

use anyhow::Result;
use imgui::Ui;
use tracy::rendering::Stream;

mod ch05;
mod ch06;
mod ch07;
mod ch08;
mod ch09;
mod ch10;
mod ch11;
mod ch12;

/// Traits shared by all the renderable scenes.
pub trait Scene {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn render(&mut self, width: u32, height: u32) -> Stream;
    fn draw(&mut self, ui: &Ui) -> bool;
}

/// Returns a list of all the available scenes.
pub fn get_scene_list() -> Result<Vec<Box<dyn Scene>>> {
    Ok(vec![
        Box::new(ch05::FlatSphere::new()?),
        Box::new(ch06::PhongSphere::new()?),
        Box::new(ch07::ThreeSpheres::new()?),
        Box::new(ch08::ShadowSpheres::new()?),
        Box::new(ch09::PlaneShape::new()?),
        Box::new(ch10::Patterns::new()?),
        Box::new(ch11::Reflections::new()?),
        Box::new(ch12::Tables::new()?),
    ])
}
