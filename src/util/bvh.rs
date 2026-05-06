use glam::DVec3;

use crate::util::{DInterval, Ray};

#[derive(Default, Copy, Clone, Debug)]
pub struct Bounds3 {
    pub min: DVec3,
    pub max: DVec3,
}

impl Bounds3 {
    pub fn new(min: DVec3, max: DVec3) -> Self {
        Self { min, max }
    }

    pub const EMPTY: Self = Self {
        min: DVec3::MAX,
        max: DVec3::MIN,
    };

    pub const UNIVERSE: Self = Self {
        min: DVec3::MIN,
        max: DVec3::MAX,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_point_inside_returns_true() {
        let pt = DVec3::ZERO;

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.contains(&pt));
    }

    #[test]
    fn test_contains_point_outside_returns_false() {
        let pt = DVec3::new(1.1, 0.0, 0.0);

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(!bounds.contains(&pt));
    }

    #[test]
    fn test_contains_point_exactly_on_min_border_returns_true() {
        let pt = DVec3::new(-1.0, -1.0, -1.0);

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.contains(&pt));
    }

    #[test]
    fn test_contains_point_exactly_on_max_border_returns_true() {
        let pt = DVec3::new(1.0, 1.0, 1.0);

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.contains(&pt));
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

        let bounds_min = DVec3::new(-1.0, -1.0, -1.0);
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
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
        let bounds_max = DVec3::new(1.0, 1.0, 1.0);
        let bounds = Bounds3::new(bounds_min, bounds_max);

        assert!(bounds.intersected_by(&ray, ray_t));
    }
}
