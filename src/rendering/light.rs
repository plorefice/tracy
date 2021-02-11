//! Light sources.

use crate::{canvas::Color, math::Point};

/// A point light source.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PointLight {
    /// Position of the light source in the world.
    pub position: Point,
    /// Color of the light source.
    pub color: Color,
    /// Brightness of the light source.
    pub intensity: f32,
}
