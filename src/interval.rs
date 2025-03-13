#[derive(Default, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn from_interval(a: &Interval, b: &Interval) -> Interval {
        // Create the interval tightly enclosing the two input intervals.
        Interval {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    // Pad interval by a given amount
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
}
