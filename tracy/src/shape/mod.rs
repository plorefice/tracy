//! Collision shapes supported by the ray tracer.

use std::fmt::Debug;

pub use cube::*;
pub use plane::*;
pub use sphere::*;

use crate::query::{AsAny, RayCast};

mod cube;
mod plane;
mod sphere;

/// Traits common to all shapes.
#[cfg_attr(feature = "serde-support", typetag::serde)]
pub trait Shape: 'static + Debug + Send + Sync + RayCast + AsAny {}
