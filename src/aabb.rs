use std::i32;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

// Struct for an axis-aligned bounding box, definde by intervals in all 3 spacial dimensions
#[derive(Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

// Constructor using intervals
pub fn aabb_from_interval(x: Interval, y: Interval, z: Interval) -> Aabb {
    Aabb { x, y, z }
}

// Constructor using two points
pub fn aabb_from_point(a: Vec3, b: Vec3) -> Aabb {
    // Treat the two points a and b as extrema for the bounding box, so we don't require a
    // particular minimum/maximum coordinate order.
    Aabb {
        x: if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        },
        y: if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        },
        z: if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        },
    }
}

impl Aabb {
    pub fn from_aabb(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            x: Interval::from_interval(&box0.x, &box1.x),
            y: Interval::from_interval(&box0.y, &box1.y),
            z: Interval::from_interval(&box0.z, &box1.z),
        }
    }

    pub fn empty() -> Aabb {
        Aabb { x: Interval::empty(), y: Interval::empty(), z: Interval::empty() }
    }

    pub fn universe() -> Aabb {
        Aabb { x: Interval::universe(), y: Interval::universe(), z: Interval::universe() }
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
}
