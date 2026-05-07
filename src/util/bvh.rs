mod bounds3;

use std::sync::Arc;

pub use bounds3::Bounds3;

use crate::{
    geom::{HitRecord, Hittable},
    util::DInterval,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BVHNode<T: Hittable> {
    children: Vec<BVHNode<T>>,
    prim: Arc<T>,
    aabb: Bounds3,
}

impl<T: Hittable> BVHNode<T> {
    fn new(prim: T) -> Self {
        let aabb = prim.aabb();
        Self {
            children: vec![],
            prim: Arc::new(prim),
            aabb,
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn add_child(&mut self, child: BVHNode<T>) {
        if self.children.len() < Self::MAX_CHILDREN {
            self.children.push(child);
            println!("constructing BVHNode AABB");
            self.aabb = self.children.iter().fold(self.prim.aabb(), |acc, x| {
                Bounds3::combined(&acc, &x.aabb())
            })
        } else {
            panic!("Attempted to add a child to an already full BVHNode");
        }
    }

    const MAX_CHILDREN: usize = 2;
}

impl<T: Hittable> Hittable for BVHNode<T> {
    fn hit(&self, ray: &super::Ray, ray_t: super::DInterval) -> Option<crate::geom::HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        // check intersection with prim
        if let Some(rec) = self
            .prim
            .hit(ray, DInterval::new(ray_t.min, closest_so_far))
        {
            closest_so_far = rec.t;
            result = Some(rec);
        }

        // check each child recursively
        for object in &self.children {
            let cur_ray_t = DInterval::new(ray_t.min, closest_so_far);
            // if intersects AABB and hits some object in heirarchy:
            if object.aabb().intersected_by(ray, cur_ray_t)
                && let Some(rec) = object.hit(ray, cur_ray_t)
            {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }
        result
    }

    fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    fn debug(&self) {
        println!(
            "BVHNode with {} children {}",
            self.children.len(),
            if self.is_leaf() { "(leaf)" } else { "" }
        )
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use glam::DVec3;

    use crate::{
        geom::Sphere,
        util::{Color, Lambertian, Ray, null_material_ptr},
    };

    use super::*;

    #[test]
    fn test_is_leaf_with_no_children_returns_true() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let node = BVHNode::new(s.clone());
        assert!(node.is_leaf());
    }

    #[test]
    fn test_is_leaf_with_one_child_returns_false() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let mut node = BVHNode::new(s.clone());
        let node2 = BVHNode::new(s);
        node.add_child(node2);
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_is_leaf_with_two_children_returns_false() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let mut node = BVHNode::new(s.clone());
        let node2 = BVHNode::new(s.clone());
        let node3 = BVHNode::new(s);
        node.add_child(node2);
        node.add_child(node3);
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_add_child_adds_child_as_long_as_theres_room() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let mut node = BVHNode::new(s.clone());
        let node2 = BVHNode::new(s.clone());
        assert_eq!(node.children.len(), 0);
        node.add_child(node2);
        assert_eq!(node.children.len(), 1);

        let node3 = BVHNode::new(s);
        node.add_child(node3);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    #[should_panic = "Attempted to add a child to an already full BVHNode"]
    fn test_add_child_panics_when_adding_a_child_with_no_room() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let mut node = BVHNode::new(s.clone());
        let node2 = BVHNode::new(s.clone());
        let node3 = BVHNode::new(s.clone());
        let node4 = BVHNode::new(s);
        node.add_child(node2);
        node.add_child(node3);
        node.add_child(node4);
    }

    #[test]
    fn test_aabb_with_no_children_returns_aabb_of_prim() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s = Sphere::new(DVec3::ZERO, 0.5, mat);

        let node = BVHNode::new(s.clone());
        assert_eq!(node.aabb(), s.aabb());
    }

    #[test]
    fn test_aabb_with_one_child_returns_combined_aabb_from_prim_and_child() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s1 = Sphere::new(DVec3::ZERO, 0.5, mat.clone());
        let s2 = Sphere::new(DVec3::new(1.0, 1.0, 1.0), 0.5, mat);

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        node.add_child(node2);

        let expected_min = DVec3::new(-0.5, -0.5, -0.5);
        let expected_max = DVec3::new(1.5, 1.5, 1.5);
        let expected_aabb = Bounds3::new(expected_min, expected_max);
        assert_eq!(node.aabb(), expected_aabb);
    }

    #[test]
    fn test_aabb_with_two_children_returns_combined_aabb_from_prim_and_both_children() {
        let mat = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
        let s1 = Sphere::new(DVec3::ZERO, 0.5, mat.clone());
        let s2 = Sphere::new(DVec3::new(1.0, 1.0, 1.0), 0.5, mat.clone());
        let s3 = Sphere::new(DVec3::new(0.0, 0.0, -2.0), 0.5, mat.clone());

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        let node3 = BVHNode::new(s3);
        node.add_child(node2);
        node.add_child(node3);

        let expected_min = DVec3::new(-0.5, -0.5, -2.5);
        let expected_max = DVec3::new(1.5, 1.5, 1.5);
        let expected_aabb = Bounds3::new(expected_min, expected_max);
        assert_eq!(node.aabb(), expected_aabb);
    }

    #[test]
    fn test_hit_with_no_children_not_intersecting_with_prim_returns_empty() {
        let s = Sphere::new(DVec3::ZERO, 0.1, null_material_ptr());

        let node = BVHNode::new(s);

        let ray = Ray::new(DVec3::ONE, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        assert!(node.hit(&ray, ray_t).is_none());
    }

    #[test]
    fn test_hit_intersects_prim_returns_prim_hit() {
        let x_loc = 0.0;
        let y_loc = 0.0;
        let z_loc = -1.0;
        let rad = 0.5;

        // TODO: add mocking so I don't have to keep calculating these things
        let s = Sphere::new(DVec3::new(x_loc, y_loc, z_loc), rad, null_material_ptr());
        let node = BVHNode::new(s);

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        let ray_hit = node.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.5);
        assert_eq!(ray_hit.point, DVec3::new(0.0, 0.0, -0.5));
        assert_eq!(ray_hit.normal, DVec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_hit_intersects_child_returns_child_hit() {
        // TODO
    }
}
