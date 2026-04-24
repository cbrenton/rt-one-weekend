use std::sync::Arc;

use crate::util::{ALMOST_ZERO, DInterval, Material, Ray};
use glam::DVec3;

use super::{HitRecord, Hittable};

pub struct Triangle {
    a: DVec3,
    b: DVec3,
    c: DVec3,
    mat: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: DVec3, b: DVec3, c: DVec3, mat: Arc<dyn Material>) -> Self {
        Self { a, b, c, mat }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;

        // NOTE: we don't normalize this because we're using the length squared as a shortcut for
        // "area of the parallelogram defined by AB and AC"
        let normal = ab.cross(ac);

        // find ray intersection with plane
        let t = {
            // NOTE: this is reversed from plane.rs, since ray.direction is unit length but normal isn't
            let denom = normal.dot(ray.direction());

            // ray is parallel to the triangle - ray projected onto normal approaches zero
            if denom.abs() < ALMOST_ZERO {
                return None;
            }

            normal.dot(self.a - ray.origin()) / denom
        };

        if !ray_t.surrounds(t) {
            return None;
        }

        let p = ray.at(t);

        // TODO: grok and document this
        let twice_abc_area = normal.length_squared();
        let twice_bcp_area = (self.c - self.b).cross(p - self.b).dot(normal);
        let twice_cap_area = (self.a - self.c).cross(p - self.c).dot(normal);
        let twice_abp_area = (self.b - self.a).cross(p - self.a).dot(normal);

        let u = twice_bcp_area / twice_abc_area;
        let v = twice_cap_area / twice_abc_area;
        let w = twice_abp_area / twice_abc_area;

        if u < 0.0 || v < 0.0 || w < 0.0 {
            return None;
        }

        let mut rec = HitRecord {
            point: ray.at(t),
            t,
            u,
            v,
            ..Default::default()
        };
        rec.set_face_normal(ray, normal.normalize());
        rec.mat = Some(self.mat.clone());
        Some(rec)
    }

    fn debug(&self) {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        println!("Triangle with points {a:?}, {b:?}, {c:?}");
    }
}
