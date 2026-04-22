mod canvas;
mod color;
mod interval;
mod ray;

pub use canvas::Canvas;
pub use color::Color;
pub use interval::{DInterval, IInterval, Interval};
pub use ray::Ray;

use rand::prelude::*;
use std::ops::Range;

// NOTE: for infinity, use f64::INFINITY
// NOTE: for pi, use f64::consts::PI
// NOTE: for degrees to radians, use f64::to_radians()

// NOTE: rust inlines small functions automatically. I assume this counts
/// Returns a double between 0.0 and 1.0.
pub fn random_double() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

/// Returns a double in the given range (exclusive).
pub fn random_double_range(range: Range<f64>) -> f64 {
    rand::rng().random_range(range)
}
