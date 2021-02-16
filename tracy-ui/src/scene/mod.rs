//! Generators for each chapter's exercises.

use imgui::Ui;
use tracy::canvas::Canvas;

mod ch02;
mod ch04;
mod ch05;
mod ch06;
mod ch07;

/// Traits shared by all the renderable scenes.
pub trait Scene {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn render(&self, width: u32, height: u32) -> Canvas;
    fn draw(&mut self, ui: &Ui) -> bool;
}

/// Returns a list of all the available scenes.
pub fn get_scene_list() -> Vec<Box<dyn Scene>> {
    vec![
        Box::new(ch02::Trajectory::default()),
        Box::new(ch04::Clock::default()),
        Box::new(ch05::FlatSphere::default()),
        Box::new(ch06::PhongSphere::default()),
        Box::new(ch07::ThreeSpheres::default()),
    ]
}
