//! Collision shapes supported by the ray tracer.

mod plane;
mod sphere;

use std::fmt::Debug;

pub use plane::*;
pub use sphere::*;

use crate::query::{AsAny, RayCast};

/// Traits common to all shapes.
#[cfg_attr(feature = "serde-support", typetag::serde)]
pub trait Shape: 'static + Debug + Send + Sync + RayCast + AsAny {}
