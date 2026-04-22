use glam::DVec3;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }

    // only allow getting origin, don't let it be updated after creation
    pub fn origin(&self) -> DVec3 {
        self.origin
    }

    // samesies
    pub fn direction(&self) -> DVec3 {
        self.direction
    }
}
