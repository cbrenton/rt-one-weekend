#![allow(dead_code)]

use glam::DVec3;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    origin: DVec3,
    direction: DVec3,
    direction_inv: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray {
            origin,
            direction,
            // calculate 1.0 / direction to let us replace expensive division with cheap
            // multiplication in aabb intersection test
            direction_inv: 1.0 / direction,
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> DVec3 {
        self.origin
    }

    pub fn direction(&self) -> DVec3 {
        self.direction
    }

    pub fn direction_inv(&self) -> DVec3 {
        self.direction_inv
    }
}
