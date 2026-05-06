mod sphere;
mod triangle;
mod triangle_mesh;

use crate::util::{Bounds3, DInterval, Material, Ray};
use glam::DVec3;
use std::{any::type_name_of_val, sync::Arc};

pub use sphere::Sphere;
pub use triangle::Triangle;
pub use triangle_mesh::TriangleMesh;

// TODO: move this elsewhere?
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord>;
    fn debug(&self);
    // TODO: is there a better way to do this? maybe construct on initialization?
    fn aabb(&mut self) -> Bounds3 {
        Bounds3::UNIVERSE
    }
}

// TODO: move this elsewhere?
#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: DVec3::ZERO,
            normal: DVec3::ZERO,
            t: 0.0,
            u: 0.0,
            v: 0.0,
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

    fn debug(&self) {
        println!("HittableList");
    }

    fn aabb(&mut self) -> Bounds3 {
        Bounds3 {
            min: self
                .objects
                .iter_mut()
                .fold(DVec3::MAX, |cur_min, obj| cur_min.min(obj.aabb().min)),
            max: self
                .objects
                .iter_mut()
                .fold(DVec3::MIN, |cur_max, obj| cur_max.max(obj.aabb().max)),
        }
    }
}
