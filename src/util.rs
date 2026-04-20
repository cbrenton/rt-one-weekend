mod canvas;
mod color;
mod ray;

use core::f32;

pub use canvas::Canvas;
pub use color::Color;
pub use ray::Ray;

const INFINITY: f32 = f32::INFINITY;
const PI: f32 = f32::consts::PI;

// NOTE: for degrees to radians, use f32::to_radians()
