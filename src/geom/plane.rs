use std::sync::Arc;

use crate::util::{DInterval, Material, Ray};
use glam::DVec3;

use super::{HitRecord, Hittable};

pub struct Plane {
    point: DVec3,
    normal: DVec3,
    mat: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: DVec3, normal: DVec3, mat: Arc<dyn Material>) -> Self {
        Self { point, normal, mat }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        let denom = ray.direction().dot(self.normal);
        let t = (self.point - ray.origin()).dot(self.normal) / denom;

        if ray_t.surrounds(t) {
            let mut rec = HitRecord::default();
            rec.t = t;
            rec.point = ray.at(rec.t);
            rec.set_face_normal(ray, self.normal);
            rec.mat = Some(self.mat.clone());
            Some(rec)
        } else {
            None
        }
    }

    fn debug(&self) {
        println!("Plane with point {} and normal {}", self.point, self.normal);
    }
}
