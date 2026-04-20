use glam::Vec3;
use crate::util::Ray;

use super::{Hittable, HitRecord};

pub struct Sphere {
    pub radius: f32,
    pub center: Vec3,
}

impl Sphere {
    pub fn new(radius: f32, center: Vec3) -> Self {
        Self { radius, center }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return false
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // find nearest root in the acceptable range
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.normal = (rec.point - self.center) / self.radius;

        true
    }
}
