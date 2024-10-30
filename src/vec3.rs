use std::ops;

use crate::color::{color, Color};
#[derive(Default, Clone)]
pub struct Vec3{
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3{
    Vec3 { x, y, z }
}

impl Vec3{
    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn to_color(&self) -> Color {
        color(self.x , self.y , self.z)
    }
}

impl ops::Add for Vec3{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Add for &Vec3{
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{
           x: rhs * self.x,
           y: rhs * self.y,
           z: rhs * self.z 
        }
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3{
           x: rhs * self.x,
           y: rhs * self.y,
           z: rhs * self.z 
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
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
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
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl std::ops::Neg<> for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3{
    v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}