use crate::vec3::Vec3;

pub fn ray(orig: Vec3, dir: Vec3) -> Ray {
    Ray { orig, dir }
}

#[derive(Default)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
