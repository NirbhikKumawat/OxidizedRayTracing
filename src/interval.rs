use crate::utility::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn new_from_intervals(int1: &Interval, int2: &Interval) -> Self {
        let min = if int1.min <= int2.min {
            int1.min
        } else {
            int2.min
        };
        let max = if int1.max >= int2.max {
            int1.max
        } else {
            int2.max
        };
        Self::new(min, max)
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    pub fn expand(&self, x: f64) -> Self {
        let padding = x / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}
pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
