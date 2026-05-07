use std::sync::Arc;

use crate::util::{ALMOST_ZERO, Bounds3, DInterval, Material, Ray};
use glam::DVec3;

use super::{HitRecord, Hittable};

pub struct Triangle {
    a: DVec3,
    b: DVec3,
    c: DVec3,
    mat: Arc<dyn Material>,
    aabb: Bounds3,
}

impl Triangle {
    pub fn new(a: DVec3, b: DVec3, c: DVec3, mat: Arc<dyn Material>) -> Self {
        let pts = [a, b, c];
        println!("constructing Triangle AABB");
        let aabb = Bounds3::new(
            pts.iter().fold(DVec3::MAX, |cur_min, &pt| cur_min.min(pt)),
            pts.iter().fold(DVec3::MIN, |cur_max, &pt| cur_max.max(pt)),
        );
        Self { a, b, c, mat, aabb }
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

    fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    fn debug(&self) {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        println!("Triangle with points {a:?}, {b:?}, {c:?}");
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::util::null_material_ptr;

    use super::*;

    #[test]
    fn test_hit_happy_path() {
        let t = Triangle::new(
            DVec3::new(-1.0, -1.0, 1.0),
            DVec3::new(-1.0, 1.0, 1.0),
            DVec3::new(1.0, 1.0, 1.0),
            null_material_ptr(),
        );

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, 1.0));
        let ray_t = DInterval::UNIVERSE;

        let ray_hit = t.hit(&ray, ray_t).unwrap();

        assert_approx_eq!(ray_hit.t, 1.0);
        assert_eq!(ray_hit.point, DVec3::new(0.0, 0.0, 1.0));
        assert_eq!(ray_hit.normal, DVec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_aabb() {
        let a = DVec3::new(-1.0, -1.0, 1.0);
        let b = DVec3::new(-1.0, 1.0, 1.0);
        let c = DVec3::new(1.0, 1.0, 1.0);

        let t = Triangle::new(a, b, c, null_material_ptr());

        let expected_min = DVec3::new(-1.0, -1.0, 1.0);
        let expected_max = DVec3::new(1.0, 1.0, 1.0);
        // TODO: figure out how to implement == for this
        assert_eq!(t.aabb().min, expected_min);
        assert_eq!(t.aabb().max, expected_max);
    }
}
