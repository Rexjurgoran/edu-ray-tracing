use crate::vec3::Vec3;
#[derive(Clone,Copy)]
pub struct Ray{
    pub orig: crate::vec3::Vec3,
    pub dir: crate::vec3::Vec3
}

impl Ray {
    pub fn origin(self) -> crate::vec3::Vec3 {
        self.orig
    }

    pub fn direction(self) -> crate::vec3::Vec3 {
        self.dir
    }

    fn at(&self, t: f64) -> crate::vec3::Vec3 {
        self.orig + self.dir * t
    }
}