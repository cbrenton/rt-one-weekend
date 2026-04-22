mod canvas;
mod color;
mod interval;
mod ray;

pub use canvas::Canvas;
pub use color::Color;
pub use interval::Interval;
pub use ray::Ray;

// NOTE: for infinity, use f64::INFINITY
// NOTE: for pi, use f64::consts::PI
// NOTE: for degrees to radians, use f64::to_radians()
