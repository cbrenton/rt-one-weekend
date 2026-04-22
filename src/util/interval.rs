use std::ops::{Add, Sub};

#[derive(Copy, Clone, Default)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy> Interval<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> T {
        self.max - self.min
    }

    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: T) -> T {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Interval<f64> {
    pub const EMPTY: Self = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}

impl Interval<i32> {
    pub const EMPTY: Self = Interval {
        min: i32::MAX,
        max: i32::MIN,
    };

    pub const UNIVERSE: Self = Interval {
        min: i32::MIN,
        max: i32::MAX,
    };
}
