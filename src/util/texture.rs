use std::sync::Arc;

use glam::DVec3;

use crate::util::Color;

pub trait Texture {
    fn value(&self, _u: f64, _v: f64, _p: DVec3) -> Color {
        Color::ZERO
    }
}

#[derive(Default, Copy, Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: DVec3) -> Color {
        self.albedo
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: even.clone(),
            odd: odd.clone(),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: DVec3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            // println!("even at {}, {}, {}", u, v, p);
            self.even.value(u, v, p)
        } else {
            // println!("odd at {}, {}, {}", u, v, p);
            self.odd.value(u, v, p)
        }
    }
}
