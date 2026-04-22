use std::ops::{Add, Mul, Sub};

pub type DInterval = Interval<f64>;
pub type IInterval = Interval<i32>;

#[derive(Copy, Clone, Default)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialOrd + Copy> Interval<T> {
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

impl DInterval {
    pub const EMPTY: Self = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Self = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// Converts range (0.0..1.0) to (min..max)
    pub fn scale(&self, x: f64) -> f64 {
        x * self.size() + self.min
    }
}

impl IInterval {
    pub const EMPTY: Self = Interval {
        min: i32::MAX,
        max: i32::MIN,
    };

    pub const UNIVERSE: Self = Interval {
        min: i32::MIN,
        max: i32::MAX,
    };

    /// Converts range (0.0..1.0) to (min..max)
    pub fn scale(&self, x: f64) -> i32 {
        (x * self.size() as f64) as i32 + self.min
    }
}
