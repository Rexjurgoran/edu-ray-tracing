use rand::seq::index;

use crate::{rtweekend::{random_double, random_int_from}, vec3::Vec3};

pub struct Perlin {
    randfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: i32 = 256;
        let mut randfloat = Vec::with_capacity(255);
        for _i in 0..point_count {
            randfloat.push(random_double());
        }
        let mut perm_x = Vec::with_capacity(255);
        let mut perm_y = Vec::with_capacity(255);
        let mut perm_z = Vec::with_capacity(255);
        Perlin::generate_perm(point_count, &mut perm_x);
        Perlin::generate_perm(point_count, &mut perm_y);
        Perlin::generate_perm(point_count, &mut perm_z);

        Perlin {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = ((4.0 * p.x) as i32 & 255) as usize;
        let j = ((4.0 * p.y) as i32 & 255) as usize;
        let k = ((4.0 * p.z) as i32 & 255) as usize;

        self.randfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn generate_perm(point_count: i32, p: &mut Vec<i32>) {
        for i in 0..point_count {
            p.push(i);
        }
        Perlin::permute(p, point_count);
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in 0..n - 1 {
            let target = random_int_from(0, i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
}
