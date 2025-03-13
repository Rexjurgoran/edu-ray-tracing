use crate::{
    rtweekend::random_int_from,
    vec3::{dot, random_from, unit_vector, vec3, Vec3},
};

pub struct Perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: i32 = 256;
        let mut randvec = Vec::with_capacity(256);
        for _i in 0..point_count {
            randvec.push(unit_vector(&random_from(-1.0, 1.0)));
        }

        Perlin {
            randvec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x - f64::floor(p.x);
        let v = p.y - f64::floor(p.y);
        let w = p.z - f64::floor(p.z);

        let i = f64::floor(p.x) as i32;
        let j = f64::floor(p.y) as i32;
        let k = f64::floor(p.z) as i32;

        let mut c = [[[vec3(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize]
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        f64::abs(accum)
    }

    fn generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);
        p
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in 0..n - 1 {
            let target = random_int_from(0, i);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = vec3(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * dot(&c[i][j][k], &weight);
                }
            }
        }
        accum
    }
}
