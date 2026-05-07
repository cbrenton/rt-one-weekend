use std::{f64::consts::PI, sync::Arc};

use crate::util::{Bounds3, DInterval, Material, Ray};
use glam::DVec3;

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
    mat: Arc<dyn Material>,
    aabb: Bounds3,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, mat: Arc<dyn Material>) -> Self {
        println!("constructing Sphere AABB");
        let aabb = Bounds3::new(center - radius, center + radius);
        Self {
            center,
            radius: f64::max(0.0, radius),
            // TODO: should I make this (and all other geometry) take the mat as a reference to an
            // Arc and clone here?
            mat,
            aabb,
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
        let d = (rec.point - self.center).normalize();
        rec.u = 0.5 + (-d.z).atan2(d.x) / (2.0 * PI);
        rec.v = 0.5 + d.y.asin() / (PI);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());

        Some(rec)
    }

    // TODO: cache this
    fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    fn debug(&self) {
        println!(
            "Sphere with radius {} at center {}",
            self.radius, self.center
        );
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::util::null_material_ptr;

    use super::*;

    #[test]
    fn test_hit_happy_path() {
        let x_loc = 0.0;
        let y_loc = 0.0;
        let z_loc = -1.0;
        let rad = 0.5;

        let s = Sphere::new(DVec3::new(x_loc, y_loc, z_loc), rad, null_material_ptr());

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        let ray_hit = s.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.5);
        assert_eq!(ray_hit.point, DVec3::new(0.0, 0.0, -0.5));
        assert_eq!(ray_hit.normal, DVec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_aabb() {
        let x_loc = 1.0;
        let y_loc = 0.0;
        let z_loc = -1.0;
        let rad = 0.5;

        let s = Sphere::new(DVec3::new(x_loc, y_loc, z_loc), rad, null_material_ptr());

        let expected_min = DVec3::new(x_loc - rad, y_loc - rad, z_loc - rad);
        let expected_max = DVec3::new(x_loc + rad, y_loc + rad, z_loc + rad);
        // TODO: figure out how to implement == for this
        assert_eq!(s.aabb(), Bounds3::new(expected_min, expected_max));
    }
}
