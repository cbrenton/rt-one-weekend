#![allow(dead_code)]

use glam::DVec3;
use mockall::automock;

use crate::util::{DInterval, Ray};

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Bounds3 {
    pub min: DVec3,
    pub max: DVec3,
    centroid: DVec3,
}

#[automock]
impl Bounds3 {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        let centroid = min + (max - min) / 2.0;
        Self { min, max, centroid }
    }

    pub fn combined(first: &Bounds3, second: &Bounds3) -> Self {
        let mut min = DVec3::ZERO;
        let mut max = DVec3::ZERO;
        for i in 0..3 {
            min[i] = first.min[i].min(second.min[i]);
            max[i] = first.max[i].max(second.max[i]);
        }
        Bounds3::new(min, max)
    }

    pub const EMPTY: Bounds3 = Bounds3 {
        min: DVec3::MAX,
        max: DVec3::MIN,
        centroid: DVec3::ZERO,
    };

    pub const UNIVERSE: Bounds3 = Bounds3 {
        min: DVec3::MIN,
        max: DVec3::MAX,
        centroid: DVec3::ZERO,
    };

    pub const UNIT: Bounds3 = Bounds3 {
        min: DVec3::NEG_ONE,
        max: DVec3::ONE,
        centroid: DVec3::ZERO,
    };

    pub fn contains(&self, pt: &DVec3) -> bool {
        for i in 0..3 {
            let dim_inside = pt[i] >= self.min[i] && pt[i] <= self.max[i];
            if !dim_inside {
                return false;
            }
        }
        true
    }

    pub fn intersected_by(&self, ray: &Ray, ray_t: DInterval) -> bool {
        let mut t_min = 0_f64;
        let mut t_max = f64::MAX;

        for i in 0..3 {
            let t1 = (self.min[i] - ray.origin()[i]) * ray.direction_inv()[i];
            let t2 = (self.max[i] - ray.origin()[i]) * ray.direction_inv()[i];
            t_min = t1.max(t_min).min(t2.max(t_min));
            t_max = t1.min(t_max).max(t2.min(t_max));
        }

        let t_min_valid = t_min >= ray_t.min && t_min <= ray_t.max;
        let t_max_valid = t_max >= ray_t.min && t_max <= ray_t.max;
        t_max >= t_min && (t_min_valid || t_max_valid)
    }

    // Get the position of a point pt relative to a bounding box, where a point at exactly
    // bounds.min has offset (0, 0, 0), a point at exactly bounds.max has offset (1, 1, 1), and a
    // point exactly halfway between min and max has offset (0.5, 0.5, 0.5).
    //
    // A point outside of the bounding box will give a scaled multiple - a point at min + 2*max
    // will have offset (2, 2, 2), and one at min - max will have offset (-1, -1, -1).
    pub fn offset(&self, pt: &DVec3) -> DVec3 {
        let mut o = pt - self.min;
        for dim in 0..3 {
            if self.max[dim] > self.min[dim] {
                o[dim] /= self.max[dim] - self.min[dim];
            }
        }
        o
    }

    pub fn centroid(&self) -> DVec3 {
        self.centroid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_uses_smaller_min_and_larger_max() {
        let first = Bounds3::UNIT;
        let second = Bounds3::new(DVec3::new(-0.9, -2.0, -1.0), DVec3::new(0.0, 1.0, 1.1));

        let result = Bounds3::combined(&first, &second);
        assert_eq!(
            result.min,
            DVec3::new(first.min.x, second.min.y, first.min.z)
        );
        assert_eq!(
            result.max,
            DVec3::new(first.max.x, first.max.y, second.max.z)
        );
    }

    #[test]
    fn test_combined_with_completely_enveloped_bounds_uses_outer_bounds() {
        let first = Bounds3::UNIT;
        let second = Bounds3::new(DVec3::new(-0.1, -0.1, -0.1), DVec3::new(0.1, 0.1, 0.1));

        let result = Bounds3::combined(&first, &second);
        assert_eq!(result.min, first.min);
        assert_eq!(result.max, first.max);
    }

    #[test]
    fn test_combined_with_empty_and_universe_returns_universe() {
        let result = Bounds3::combined(&Bounds3::EMPTY, &Bounds3::UNIVERSE);
        assert_eq!(result.min, Bounds3::UNIVERSE.min);
        assert_eq!(result.max, Bounds3::UNIVERSE.max);
    }

    #[test]
    fn test_contains_point_inside_returns_true() {
        assert!(Bounds3::UNIT.contains(&DVec3::ZERO));
    }

    #[test]
    fn test_contains_point_outside_returns_false() {
        let pt = DVec3::new(1.1, 0.0, 0.0);

        assert!(!Bounds3::UNIT.contains(&pt));
    }

    #[test]
    fn test_contains_point_exactly_on_min_border_returns_true() {
        assert!(Bounds3::UNIT.contains(&DVec3::NEG_ONE));
    }

    #[test]
    fn test_contains_point_exactly_on_max_border_returns_true() {
        assert!(Bounds3::UNIT.contains(&DVec3::ONE));
    }

    #[test]
    fn test_intersected_by_intersecting_ray_returns_true() {
        let ray_origin = DVec3::ZERO;
        let ray_dir = DVec3::new(0.0, 0.0, 1.0);
        let ray_t = DInterval::UNIVERSE;
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::new(-1.0, -1.0, 1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 3.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_intersected_by_non_intersecting_ray_returns_false() {
        let ray_origin = DVec3::new(-1.5, 0.0, 0.0);
        let ray_dir = DVec3::new(0.0, 0.0, 1.0);
        let ray_t = DInterval::UNIVERSE;
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::new(-1.0, -1.0, 1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 3.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(!bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_intersected_by_ray_exactly_on_border_returns_true() {
        let ray_origin = DVec3::new(-1.0, 0.0, 0.0);
        let ray_dir = DVec3::new(0.0, 0.0, 1.0);
        let ray_t = DInterval::UNIVERSE;
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::new(-1.0, -1.0, 1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 3.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_intersected_by_ray_exactly_on_corner_returns_true() {
        let ray_origin = DVec3::new(0.0, 2.0, 0.0);
        let ray_dir = DVec3::new(1.0, -1.0, -1.0);
        let ray_t = DInterval::new(0.0, 10.0);
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::NEG_ONE;
        let bounds_max = DVec3::ONE;
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_intersected_by_ray_outside_of_ray_t_returns_false() {
        let ray_origin = DVec3::ZERO;
        let ray_dir = DVec3::new(0.0, 0.0, 1.0);
        let ray_t = DInterval::new(100.0, 1000.0);
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::new(-1.0, -1.0, 1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 3.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(!bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_intersected_by_ray_with_one_intersection_outside_of_ray_t_returns_true() {
        let ray_origin = DVec3::ZERO;
        let ray_dir = DVec3::new(0.0, 0.0, 1.0);
        let ray_t = DInterval::new(0.0, 2.0);
        let ray = Ray::new(ray_origin, ray_dir);

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::ONE;
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.intersected_by(&ray, ray_t));
    }

    #[test]
    fn test_offset_pt_at_min_returns_zeroes() {
        let bounds = Bounds3::UNIT;

        let pt = bounds.min;

        assert_eq!(bounds.offset(&pt), DVec3::ZERO);
    }

    #[test]
    fn test_offset_pt_at_max_returns_ones() {
        let bounds = Bounds3::UNIT;

        let pt = bounds.max;

        assert_eq!(bounds.offset(&pt), DVec3::ONE);
    }

    #[test]
    fn test_offset_pt_at_center_returns_point_fives() {
        let bounds = Bounds3::UNIT;

        let pt = bounds.centroid();

        assert_eq!(bounds.offset(&pt), DVec3::splat(0.5));
    }

    #[test]
    fn test_offset_pt_not_directly_in_between_min_and_max_returns_correct_value() {
        let bounds = Bounds3::UNIT;

        let pt = DVec3::new(-1.0, 1.0, 3.0);

        assert_eq!(bounds.offset(&pt), DVec3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn test_offset_pt_before_min_returns_scaled_value() {
        let bounds = Bounds3::UNIT;

        let pt = DVec3::splat(-3.0);

        assert_eq!(bounds.offset(&pt), DVec3::NEG_ONE);
    }

    #[test]
    fn test_offset_pt_past_max_returns_scaled_value() {
        let bounds = Bounds3::UNIT;

        let pt = DVec3::splat(3.0);

        assert_eq!(bounds.offset(&pt), DVec3::splat(2.0));
    }

    #[test]
    fn test_centroid_returns_correct_value() {
        // TODO: test NEG_INF/INF
        let bounds = Bounds3::new(DVec3::NEG_ONE, DVec3::ONE);
        assert_eq!(bounds.centroid(), DVec3::ZERO);

        let bounds2 = Bounds3::new(DVec3::ZERO, DVec3::ONE);
        assert_eq!(bounds2.centroid(), DVec3::splat(0.5));
    }

    #[test]
    fn test_centroid_universe_returns_origin() {
        let bounds = Bounds3::UNIVERSE;
        assert_eq!(bounds.centroid(), DVec3::ZERO);
    }

    #[test]
    fn test_centroid_one_bound_infinite_returns_infinity() {
        let infinite = Bounds3::new(DVec3::ZERO, DVec3::INFINITY);
        assert_eq!(infinite.centroid(), DVec3::INFINITY);

        let neg_infinite = Bounds3::new(DVec3::ZERO, DVec3::NEG_INFINITY);
        assert_eq!(neg_infinite.centroid(), DVec3::NEG_INFINITY);
    }
}
