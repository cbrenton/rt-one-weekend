mod bounds3;

use std::sync::Arc;

pub use bounds3::Bounds3;

use crate::geom::Hittable;

pub struct BVHNode<T: Hittable> {
    children: Vec<BVHNode<T>>,
    prim: Arc<T>,
}

impl<T: Hittable> BVHNode<T> {
    fn new(prim: T) -> Self {
        Self {
            children: vec![],
            prim: Arc::new(prim),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn add_child(&mut self, child: BVHNode<T>) {
        if self.children.len() < Self::MAX_CHILDREN {
            self.children.push(child);
        } else {
            panic!("Attempted to add a child to an already full BVHNode");
        }
    }

    const MAX_CHILDREN: usize = 2;
}

impl<T: Hittable> Hittable for BVHNode<T> {
    fn hit(&self, ray: &super::Ray, ray_t: super::DInterval) -> Option<crate::geom::HitRecord> {
        todo!();
    }

    fn aabb(&self) -> Bounds3 {
        // TODO: cache this on child add
        println!("constructing BVHNode AABB");
        self.children.iter().fold(self.prim.aabb(), |acc, x| {
            Bounds3::combined(&acc, &x.aabb())
        })
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
    use glam::DVec3;

    use crate::{
        geom::Sphere,
        util::{Color, Lambertian},
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
}
