use std::sync::Arc;

use crate::util::{DInterval, Material, Ray};
use glam::DVec3;

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: DVec3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // find nearest root in the acceptable range
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());

        Some(rec)
    }
}
