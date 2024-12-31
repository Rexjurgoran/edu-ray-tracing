use std::ops;

use crate::{
    color::{color, Color},
    rtweekend::{random_double, random_double_from},
};
#[derive(Default, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn to_color(&self) -> Color {
        color(self.x, self.y, self.z)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl std::ops::Mul<&Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self / rhs as f64
    }
}

impl std::ops::Div<i32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self / rhs as f64
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    vec3(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z, 
        u.x * v.y - u.y * v.x)
}

pub fn random() -> Vec3 {
    vec3(random_double(), random_double(), random_double())
}

pub fn random_from(min: f64, max: f64) -> Vec3 {
    vec3(
        random_double_from(min, max),
        random_double_from(min, max),
        random_double_from(min, max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_from(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / f64::sqrt(lensq);
        }
    }
}

pub fn random_in_unit_disk() -> Vec3{
    loop {
        let p = vec3(random_double_from(-1.0, 1.0), random_double_from(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
//     let on_unit_sphere = random_unit_vector();
//     if dot(&on_unit_sphere, normal) > 0.0 {
//         return on_unit_sphere;
//     } else {
//         return -on_unit_sphere;
//     }
// }

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2.0 * dot(v, n) * n;
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}