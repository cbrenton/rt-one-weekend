#![allow(dead_code)]

mod bounds3;

use std::sync::Arc;

pub use bounds3::Bounds3;

use crate::{
    geom::{HitRecord, Hittable},
    util::{DInterval, Ray},
};

#[derive(Clone)]
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

    fn intersected_by(&self, ray: &Ray, ray_t: DInterval) -> bool {
        self.aabb.intersected_by(ray, ray_t)
    }

    fn aabb(&self) -> Bounds3 {
        self.aabb
    }

    const MAX_CHILDREN: usize = 2;
}

impl<T: Hittable> Hittable for BVHNode<T> {
    fn hit(&self, ray: &super::Ray, ray_t: super::DInterval) -> Option<HitRecord> {
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
            if object.intersected_by(ray, cur_ray_t)
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
    use mockall::predicate;

    use crate::{
        geom::{MockHittable, Sphere},
        util::{Color, Lambertian, Ray, bvh::bounds3::MockBounds3, null_material_ptr},
    };

    use super::*;

    #[test]
    fn test_is_leaf_with_no_children_returns_true() {
        let mut s = MockHittable::new();
        s.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let node = BVHNode::new(s);
        assert!(node.is_leaf());
    }

    #[test]
    fn test_is_leaf_with_one_child_returns_false() {
        let mut s1 = MockHittable::new();
        let mut s2 = MockHittable::new();
        s1.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s2.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        node.add_child(node2);
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_is_leaf_with_two_children_returns_false() {
        let mut s1 = MockHittable::new();
        let mut s2 = MockHittable::new();
        let mut s3 = MockHittable::new();
        s1.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s2.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s3.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        let node3 = BVHNode::new(s3);
        node.add_child(node2);
        node.add_child(node3);
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_add_child_adds_child_as_long_as_theres_room() {
        let mut s1 = MockHittable::new();
        let mut s2 = MockHittable::new();
        let mut s3 = MockHittable::new();
        s1.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s2.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s3.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        let node3 = BVHNode::new(s3);

        assert_eq!(node.children.len(), 0);
        node.add_child(node2);
        assert_eq!(node.children.len(), 1);
        node.add_child(node3);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    #[should_panic = "Attempted to add a child to an already full BVHNode"]
    fn test_add_child_panics_when_adding_a_child_with_no_room() {
        let mut s1 = MockHittable::new();
        let mut s2 = MockHittable::new();
        let mut s3 = MockHittable::new();
        let mut s4 = MockHittable::new();

        s1.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s2.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s3.expect_aabb().returning(|| Bounds3::UNIVERSE);
        s4.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let mut node = BVHNode::new(s1);
        let node2 = BVHNode::new(s2);
        let node3 = BVHNode::new(s3);
        let node4 = BVHNode::new(s4);
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
        let mut s1 = MockHittable::new();
        s1.expect_aabb()
            .returning(|| Bounds3::new(DVec3::new(-0.5, -0.5, -0.5), DVec3::new(0.5, 0.5, 0.5)));

        let mut s2 = MockHittable::new();
        s2.expect_aabb()
            .returning(|| Bounds3::new(DVec3::new(0.5, 0.5, 0.5), DVec3::new(1.5, 1.5, 1.5)));

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
        let mut s1 = MockHittable::new();
        s1.expect_aabb()
            .returning(|| Bounds3::new(DVec3::new(-0.5, -0.5, -0.5), DVec3::new(0.5, 0.5, 0.5)));

        let mut s2 = MockHittable::new();
        s2.expect_aabb()
            .returning(|| Bounds3::new(DVec3::new(0.5, 0.5, 0.5), DVec3::new(1.5, 1.5, 1.5)));

        let mut s3 = MockHittable::new();
        s3.expect_aabb()
            .returning(|| Bounds3::new(DVec3::new(-0.5, -0.5, -2.5), DVec3::new(1.5, 1.5, 1.5)));

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
        let mut s = MockHittable::new();
        s.expect_hit().returning(|_ray, _ray_t| None);
        s.expect_aabb().returning(|| Bounds3::EMPTY);

        let node = BVHNode::new(s);

        let ray = Ray::new(DVec3::ONE, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        assert!(node.hit(&ray, ray_t).is_none());
    }

    #[test]
    fn test_hit_intersects_prim_returns_prim_hit() {
        let mut root_geom = MockHittable::new();

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        // expect prim.hit gets called and returns true
        root_geom
            .expect_hit()
            .with(predicate::eq(ray), predicate::eq(ray_t))
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.69,
                    point: DVec3::new(0.1, 0.2, 0.3),
                    normal: DVec3::new(-1.0, -1.0, -1.0),
                    ..Default::default()
                })
            });

        // expect child intersects gets called and returns false
        root_geom.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let node = BVHNode::new(root_geom);

        let ray_hit = node.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.69);
        assert_eq!(ray_hit.point, DVec3::new(0.1, 0.2, 0.3));
        assert_eq!(ray_hit.normal, DVec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_hit_intersects_one_child_returns_child_hit() {
        let mut root_geom = MockHittable::new();
        let mut child_geom = MockHittable::new();

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        // expect tri.hit gets called and returns true
        root_geom
            .expect_hit()
            .with(predicate::eq(ray), predicate::eq(ray_t))
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.69,
                    point: DVec3::new(0.1, 0.2, 0.3),
                    normal: DVec3::new(-1.0, -1.0, -1.0),
                    ..Default::default()
                })
            });

        child_geom
            .expect_hit()
            // TODO: is this overkill/not useful?
            .with(
                predicate::eq(ray),
                // expects ray_t.max to me updated to the max from first geom hit
                predicate::eq(DInterval::new(ray_t.min, 0.69)),
            )
            .returning(|_ray: &Ray, _ray_t: DInterval| None);

        // expect child intersects gets called and returns false
        root_geom.expect_aabb().returning(|| Bounds3::UNIVERSE);
        child_geom.expect_aabb().returning(|| Bounds3::EMPTY);

        let mut root_node = BVHNode::new(root_geom);
        let child_node = BVHNode::new(child_geom);
        root_node.add_child(child_node);

        let ray_hit = root_node.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.69);
        assert_eq!(ray_hit.point, DVec3::new(0.1, 0.2, 0.3));
        assert_eq!(ray_hit.normal, DVec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_hit_intersects_prim_and_one_child_but_misses_child_aabb_records_prim_hit() {
        let mut root_geom = MockHittable::new();
        let mut child_geom = MockHittable::new();

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        // expect tri.hit gets called and returns true
        root_geom
            .expect_hit()
            .with(predicate::eq(ray), predicate::eq(ray_t))
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.69,
                    point: DVec3::new(0.1, 0.2, 0.3),
                    normal: DVec3::new(-1.0, -1.0, -1.0),
                    ..Default::default()
                })
            });

        child_geom
            .expect_hit()
            // TODO: is this overkill/not useful?
            .with(
                predicate::eq(ray),
                // expects ray_t.max to me updated to the max from first geom hit
                predicate::eq(DInterval::new(ray_t.min, 0.69)),
            )
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.69,
                    point: DVec3::new(0.1, 0.2, 0.3),
                    normal: DVec3::new(-1.0, -1.0, -1.0),
                    ..Default::default()
                })
            });

        // expect child intersects gets called and returns false
        root_geom.expect_aabb().returning(|| Bounds3::UNIVERSE);
        child_geom.expect_aabb().returning(|| Bounds3::EMPTY);

        let mut root_node = BVHNode::new(root_geom);
        let child_node = BVHNode::new(child_geom);
        root_node.add_child(child_node);

        let ray_hit = root_node.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.69);
        assert_eq!(ray_hit.point, DVec3::new(0.1, 0.2, 0.3));
        assert_eq!(ray_hit.normal, DVec3::new(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_hit_intersects_both_children_returns_closest_child_hit() {
        let mut root_geom = MockHittable::new();
        let mut child_geom = MockHittable::new();

        let ray = Ray::new(DVec3::ZERO, DVec3::new(0.0, 0.0, -1.0));
        let ray_t = DInterval::UNIVERSE;

        // expect tri.hit gets called and returns true
        root_geom
            .expect_hit()
            .with(predicate::eq(ray), predicate::eq(ray_t))
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.69,
                    point: DVec3::new(0.1, 0.2, 0.3),
                    normal: DVec3::new(-1.0, -1.0, -1.0),
                    ..Default::default()
                })
            });

        child_geom
            .expect_hit()
            // TODO: is this overkill/not useful?
            .with(
                predicate::eq(ray),
                // expects ray_t.max to me updated to the max from first geom hit
                predicate::eq(DInterval::new(ray_t.min, 0.69)),
            )
            .returning(|_ray: &Ray, _ray_t: DInterval| {
                Some(HitRecord {
                    mat: None,
                    t: 0.65,
                    point: DVec3::new(0.11, 0.2, 0.3),
                    normal: DVec3::new(-1.1, -1.0, -1.0),
                    ..Default::default()
                })
            });

        // expect child intersects gets called and returns false
        root_geom.expect_aabb().returning(|| Bounds3::UNIVERSE);
        child_geom.expect_aabb().returning(|| Bounds3::UNIVERSE);

        let mut root_node = BVHNode::new(root_geom);
        let child_node = BVHNode::new(child_geom);
        root_node.add_child(child_node);

        let ray_hit = root_node.hit(&ray, ray_t).unwrap();
        assert_approx_eq!(ray_hit.t, 0.65);
        assert_eq!(ray_hit.point, DVec3::new(0.11, 0.2, 0.3));
        assert_eq!(ray_hit.normal, DVec3::new(-1.1, -1.0, -1.0));
    }
}
