use std::ops;
#[derive(Clone,Copy)]
pub struct Vec3{
    pub x: f64, 
    pub y: f64, 
    pub z: f64
}

impl Vec3{
    fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
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

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Vec3> for i32 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
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

pub fn unit_vector(v: Vec3) -> Vec3{
    v / v.length()
}