//! Geometric queries for ray tracing.

mod object;
mod ray;
mod world;

use std::any::Any;

pub use object::*;
pub use ray::*;
pub use world::*;

/// A trait for converting a type into a `&dyn Any`.
pub trait AsAny {
    /// Converts `self` to `&dyn Any`.
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
