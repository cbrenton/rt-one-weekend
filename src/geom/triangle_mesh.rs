#![allow(dead_code)]

use std::sync::Arc;

use crate::{
    geom::Triangle,
    util::{ALMOST_ZERO, Bounds3, DInterval, Material, Ray},
};
use glam::{DVec3, IVec3};

use super::{HitRecord, Hittable};

pub struct TriangleMesh {
    vertices: Vec<DVec3>,
    triangles: Vec<IVec3>,
    is_inlined: bool,
    cache: Vec<Triangle>,
    mat: Arc<dyn Material>,
    aabb: Bounds3,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<DVec3>,
        triangles: Vec<IVec3>,
        is_inlined: bool,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut cache: Vec<Triangle> = vec![];
        for triangle in &triangles {
            let a = vertices[triangle.x as usize];
            let b = vertices[triangle.y as usize];
            let c = vertices[triangle.z as usize];
            let tri = Triangle::new(a, b, c, mat.clone());
            cache.push(tri);
        }
        println!("constructing TriangleMesh AABB");
        let aabb = match is_inlined {
            true => Bounds3::UNIVERSE,
            false => Bounds3 {
                min: cache
                    .iter_mut()
                    .fold(DVec3::MAX, |cur_min, tri| cur_min.min(tri.aabb().min)),
                max: cache
                    .iter_mut()
                    .fold(DVec3::MIN, |cur_max, tri| cur_max.max(tri.aabb().max)),
            },
        };

        Self {
            vertices,
            triangles,
            is_inlined,
            cache,
            mat,
            aabb,
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

        if self.is_inlined {
            for triangle in &self.triangles {
                let a = self.vertices[triangle.x as usize];
                let b = self.vertices[triangle.y as usize];
                let c = self.vertices[triangle.z as usize];

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
            }
        } else {
            for triangle in &self.cache {
                if let Some(rec) = triangle.hit(ray, DInterval::new(ray_t.min, closest_so_far)) {
                    closest_so_far = rec.t;
                    result = Some(rec);
                }
            }
        }
        result
    }

    // TODO: cache this
    fn aabb(&self) -> Bounds3 {
        // TODO: make this work when inlined
        if self.is_inlined {
            todo!();
        } else {
            self.aabb
        }
    }

    fn debug(&self) {
        println!(
            "TriangleMesh with {} vertices and {} total triangles. {} triangles cached",
            self.vertices.len(),
            self.triangles.len(),
            self.cache.len(),
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
        let a = DVec3::new(-1.0, -1.0, 1.0);
        let b = DVec3::new(-1.0, 1.0, 1.0);
        let c = DVec3::new(1.0, 1.0, 1.0);
        let d = DVec3::new(1.0, -1.0, 1.0);

        let mesh = TriangleMesh::new(
            vec![a, b, c, d],
            vec![IVec3::new(0, 1, 2), IVec3::new(2, 3, 0)],
            false,
            null_material_ptr(),
        );

        let ray_tri_1 = Ray::new(DVec3::ZERO, DVec3::new(-0.5, 0.5, 1.0));
        let ray_hit_1 = mesh.hit(&ray_tri_1, DInterval::UNIVERSE).unwrap();

        assert_approx_eq!(ray_hit_1.t, 1.0);
        assert_eq!(ray_hit_1.point, DVec3::new(-0.5, 0.5, 1.0));
        assert_eq!(ray_hit_1.normal, DVec3::new(0.0, 0.0, -1.0));

        let ray_tri_2 = Ray::new(DVec3::ZERO, DVec3::new(0.5, -0.5, 1.0));
        let ray_hit_2 = mesh.hit(&ray_tri_2, DInterval::UNIVERSE).unwrap();

        assert_approx_eq!(ray_hit_2.t, 1.0);
        assert_eq!(ray_hit_2.point, DVec3::new(0.5, -0.5, 1.0));
        assert_eq!(ray_hit_2.normal, DVec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_aabb() {
        let a = DVec3::new(-1.0, -1.0, 1.0);
        let b = DVec3::new(-1.0, 1.0, 1.0);
        let c = DVec3::new(1.0, 1.0, 1.0);
        let d = DVec3::new(1.0, -1.0, 1.0);

        let mesh = TriangleMesh::new(
            vec![a, b, c, d],
            vec![IVec3::new(0, 1, 2), IVec3::new(2, 3, 0)],
            false,
            null_material_ptr(),
        );

        let expected_min = DVec3::new(-1.0, -1.0, 1.0);
        let expected_max = DVec3::new(1.0, 1.0, 1.0);
        // TODO: figure out how to implement == for this
        assert_eq!(mesh.aabb().min, expected_min);
        assert_eq!(mesh.aabb().max, expected_max);
    }
}
