//! Collision shapes supported by the ray tracer.

mod sphere;

use std::fmt::Debug;

pub use sphere::*;

use crate::query::{AsAny, RayCast};

/// Traits common to all shapes.
pub trait Shape: 'static + Debug + RayCast + AsAny {}
