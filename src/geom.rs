mod sphere;

use crate::util::{DInterval, Material, Ray};
use glam::DVec3;
use std::{mem::swap, sync::Arc};

pub use sphere::Sphere;

// TODO: move this elsewhere?
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord>;
}

// TODO: move this elsewhere?
#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: DVec3::ZERO,
            normal: DVec3::ZERO,
            t: 0.0,
            front_face: false,
            mat: None,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: DVec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, other: impl Hittable + 'static) {
        self.objects.push(Box::new(other));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, DInterval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }
}
