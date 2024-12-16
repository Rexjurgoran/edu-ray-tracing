use crate::vec3::Vec3;

pub fn ray(orig: Vec3, dir: Vec3) -> Ray {
    Ray { orig, dir, tm: 0.0 }
}

pub fn ray_with_time(orig: Vec3, dir: Vec3, tm: f64) -> Ray {
    Ray { orig, dir, tm }
}

#[derive(Default)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + self.dir.clone() * t
    }
}
