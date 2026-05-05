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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::util::{Color, Lambertian};

    use super::*;

    #[test]
    fn test_hit_happy_path_camera_facing() {
        let point = DVec3::new(1.0, 1.0, 1.0);
        let normal = DVec3::new(0.0, 0.0, -1.0);
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));

        let p = Plane::new(point, normal, mat);

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, 1.0));
        let ray_t = DInterval::UNIVERSE;

        let ray_hit = p.hit(&ray, ray_t).unwrap();

        assert_approx_eq!(ray_hit.t, 1.0);
        assert_eq!(ray_hit.point, DVec3::new(0.0, 0.0, 1.0));
        assert_eq!(ray_hit.normal, p.normal);
    }
}
