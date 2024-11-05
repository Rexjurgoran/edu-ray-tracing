pub struct Interval {
    pub min: f64,
    pub max: f64
}

pub fn interval(min: f64, max: f64) -> Interval {
    Interval { min, max }
}

impl Interval {
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}