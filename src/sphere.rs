use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            t: Default::default(),
            front_face: Default::default(),
            mat: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // Note: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    mat: Material,
}

pub fn sphere(center: Vec3, radius: f64, mat: Material) -> Sphere {
    Sphere {
        center,
        radius,
        mat,
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = &self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
