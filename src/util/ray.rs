use glam::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    // only allow getting origin, don't let it be updated after creation
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    // samesies
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
