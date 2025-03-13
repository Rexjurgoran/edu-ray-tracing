use rand::seq::index;

use crate::{
    rtweekend::{random_double, random_int_from},
    vec3::Vec3,
};

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
        let mut u = p.x - f64::floor(p.x);
        let mut v = p.y - f64::floor(p.y);
        let mut w = p.z - f64::floor(p.z);

        // Hermitian smoothing
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = f64::floor(p.x) as usize;
        let j = f64::floor(p.y) as usize;
        let k = f64::floor(p.z) as usize;

        let mut c = [[[1.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[(self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255])
                        as usize]
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
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

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}
