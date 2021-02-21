//! Generators for each chapter's exercises.

use anyhow::Result;
use imgui::Ui;
use tracy::rendering::{Canvas, Stream};

mod ch02;
mod ch04;
mod ch05;
mod ch06;
mod ch07;
mod ch08;
mod ch09;
mod ch10;
pub mod ch11;

/// Traits shared by all the renderable scenes.
pub trait Scene {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn render(&self, width: u32, height: u32) -> Result<Canvas>;
    fn draw(&mut self, ui: &Ui) -> bool;
}

/// Traits shared by all the renderable scenes.
pub trait Streamer {
    fn stream(&self, width: u32, height: u32) -> Stream;
}

/// Returns a list of all the available scenes.
pub fn get_scene_list() -> Vec<Box<dyn Scene>> {
    vec![
        Box::new(ch02::Trajectory::default()),
        Box::new(ch04::Clock::default()),
        Box::new(ch05::FlatSphere::default()),
        Box::new(ch06::PhongSphere::default()),
        Box::new(ch07::ThreeSpheres::default()),
        Box::new(ch08::ShadowSpheres::default()),
        Box::new(ch09::PlaneShape::default()),
        Box::new(ch10::Patterns::default()),
        Box::new(ch11::Reflections::new().unwrap()),
    ]
}
