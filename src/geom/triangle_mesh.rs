use std::sync::Arc;

use crate::{
    geom::Triangle,
    util::{ALMOST_ZERO, DInterval, Material, Ray},
};
use glam::{DVec3, IVec3};

use super::{HitRecord, Hittable};

pub struct TriangleMesh {
    vertices: Vec<DVec3>,
    triangles: Vec<IVec3>,
    is_inlined: bool,
    mat: Arc<dyn Material>,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<DVec3>,
        triangles: Vec<IVec3>,
        is_inlined: bool,
        mat: Arc<dyn Material>,
    ) -> Self {
        Self {
            vertices,
            triangles,
            is_inlined,
            mat,
        }
    }

    fn hit_tri(
        &self,
        ray: &Ray,
        ray_t: DInterval,
        a: &DVec3,
        b: &DVec3,
        c: &DVec3,
        mat: Arc<dyn Material>,
    ) -> Option<HitRecord> {
        let ab = b - a;
        let ac = c - a;

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

            normal.dot(a - ray.origin()) / denom
        };

        if !ray_t.surrounds(t) {
            return None;
        }

        let p = ray.at(t);

        // TODO: grok and document this
        let twice_abc_area = normal.length_squared();
        let twice_bcp_area = (c - b).cross(p - b).dot(normal);
        let twice_cap_area = (a - c).cross(p - c).dot(normal);
        let twice_abp_area = (b - a).cross(p - a).dot(normal);

        let u = twice_bcp_area / twice_abc_area;
        let v = twice_cap_area / twice_abc_area;
        let w = twice_abp_area / twice_abc_area;

        if u < 0.0 || v < 0.0 || w < 0.0 {
            return None;
        }

        let mut rec = HitRecord {
            point: ray.at(t),
            t,
            ..Default::default()
        };
        rec.set_face_normal(ray, normal.normalize());
        rec.mat = Some(mat.clone());
        Some(rec)
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: &Ray, ray_t: DInterval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for triangle in &self.triangles {
            let a = self.vertices[triangle.x as usize];
            let b = self.vertices[triangle.y as usize];
            let c = self.vertices[triangle.z as usize];

            if self.is_inlined {
                if let Some(rec) = self.hit_tri(
                    ray,
                    DInterval::new(ray_t.min, closest_so_far),
                    &a,
                    &b,
                    &c,
                    self.mat.clone(),
                ) {
                    closest_so_far = rec.t;
                    result = Some(rec);
                }
            } else {
                let triangle = Triangle::new(a, b, c, self.mat.clone());
                if let Some(rec) = triangle.hit(ray, DInterval::new(ray_t.min, closest_so_far)) {
                    closest_so_far = rec.t;
                    result = Some(rec);
                }
            }
        }
        result
    }

    fn debug(&self) {
        println!(
            "TriangleMesh with {} vertices and {} total triangles",
            self.vertices.len(),
            self.triangles.len()
        );
    }
}
