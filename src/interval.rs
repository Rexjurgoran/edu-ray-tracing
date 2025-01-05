pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub fn interval(min: f64, max: f64) -> Interval {
    Interval { min, max }
}

impl Interval {
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
        interval(self.min - padding, self.max + padding)
    }
}
