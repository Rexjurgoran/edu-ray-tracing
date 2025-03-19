use std::i32;

use crate::{interval::Interval, ray::Ray, vec3::Vec3};

// Struct for an axis-aligned bounding box, definde by intervals in all 3 spacial dimensions
#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn from_interval(x: Interval, y: Interval, z: Interval) -> Aabb {
        let mut aabb = Aabb { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_point(a: &Vec3, b: &Vec3) -> Aabb {
        // Treat the two points a and b as extrema for the bounding box, so whe don't require a
        // particular minimum/maximum coordinate oder.

        let x = Interval::new(f64::min(a.x, b.x), f64::max(a.x, b.x));
        let y = Interval::new(f64::min(a.y, b.y), f64::max(a.y, b.y));
        let z = Interval::new(f64::min(a.z, b.z), f64::max(a.z, b.z));

        let mut aabb = Aabb { x, y, z };

        aabb.pad_to_minimums();

        aabb
    }

    pub fn from_aabb(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            x: Interval::from_interval(&box0.x, &box1.x),
            y: Interval::from_interval(&box0.y, &box1.y),
            z: Interval::from_interval(&box0.z, &box1.z),
        }
    }

    pub fn empty() -> Aabb {
        Aabb {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn universe() -> Aabb {
        Aabb {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    pub fn longest_axis(&self) -> i32 {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        } else {
            return if self.y.size() > self.z.size() { 1 } else { 2 };
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            i32::MIN..=i32::MAX => &self.x,
        }
    }

    pub fn hit(&self, r: Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let ray_orig_dim = match axis {
                1 => ray_orig.y,
                2 => ray_orig.z,
                i32::MIN..=i32::MAX => ray_orig.x,
            };
            let ray_dir_dim = match axis {
                1 => ray_dir.y,
                2 => ray_dir.z,
                i32::MIN..=i32::MAX => ray_dir.x,
            };
            let adinv = 1.0 / ray_dir_dim;

            let t0 = (ax.min - ray_orig_dim) * adinv;
            let t1 = (ax.max - ray_orig_dim) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0
                }
                if t1 < ray_t.max {
                    ray_t.max = t1
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1
                }
                if t0 < ray_t.max {
                    ray_t.max = t0
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

impl std::ops::Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::from_interval(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::from_interval(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
