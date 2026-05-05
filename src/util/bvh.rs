use glam::DVec3;

use crate::geom::Hittable;

#[derive(Default, Copy, Clone, Debug)]
pub struct Bounds3 {
    pub min: DVec3,
    pub max: DVec3,
}

impl Bounds3 {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }

    pub const EMPTY: Self = Self {
        min: DVec3::MAX,
        max: DVec3::MIN,
    };

    pub const UNIVERSE: Self = Self {
        min: DVec3::MIN,
        max: DVec3::MAX,
    };

    pub fn from(object: &dyn Hittable) -> Self {
        // TODO: use Hittable.aabb
        Self::EMPTY
    }

    pub fn inside(&self, pt: &DVec3) -> bool {
        let x_inside = pt.x >= self.min.x && pt.x <= self.max.x;
        let y_inside = pt.y >= self.min.y && pt.y <= self.max.y;
        let z_inside = pt.z >= self.min.z && pt.z <= self.max.z;
        return x_inside && y_inside && z_inside;
    }
}
