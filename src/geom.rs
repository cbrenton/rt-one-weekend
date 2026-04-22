mod sphere;

use crate::util::{DInterval, Ray};
use glam::DVec3;

pub use sphere::Sphere;

// TODO: move this elsewhere?
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: DInterval, rec: &mut HitRecord) -> bool;
}

// TODO: move this elsewhere?
#[derive(Default, Copy, Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: DVec3, normal: DVec3, t: f64, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }

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
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, other: impl Hittable + 'static) {
        self.objects.push(Box::new(other));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: DInterval, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, DInterval::new(ray_t.min, closest_so_far), &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec;
            }
        }
        hit_anything
    }
}
