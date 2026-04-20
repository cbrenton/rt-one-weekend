use crate::util::Ray;
use glam::Vec3;

mod sphere;

pub use sphere::Sphere;

// TODO: move this elsewhere?
pub trait Hittable {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool;
}

// TODO: move this elsewhere?
#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, front_face: bool) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
