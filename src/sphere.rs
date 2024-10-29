use crate::{ray::Ray, vec3::{dot, Vec3}};

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord{
    pub fn setFaceNormal(mut self, r: Ray, outward_normal: Vec3){
        // Sets the hit record normal vector.
        // Note: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}
 
pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn hit(self, r: Ray, ray_tmin: f64, ray_tmax: f64, mut rec: HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false
        }
        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false}
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.setFaceNormal(r, outward_normal);

        true
    }
}