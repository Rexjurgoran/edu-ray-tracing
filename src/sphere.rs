
use std::f64::consts::PI;

use crate::{
    aabb::{aabb_from_point, Aabb}, interval::Interval, material::Material, ray::{ray, Ray}, vec3::{dot, vec3, Vec3}
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &Aabb;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Default::default(),
            normal: Default::default(),
            t: Default::default(),
            u: Default::default(),
            v: Default::default(),
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
    pub center: Ray,
    pub radius: f64,
    mat: Material,
    bbox: Aabb,
}
impl Sphere {
    // Stationary Sphere
    pub fn new(center: Vec3, radius: f64, mat: Material) -> Sphere {
        let rvec = vec3(radius, radius, radius);
        Sphere {
            center: ray(center.clone(), vec3(0.0, 0.0, 0.0)),
            radius,
            mat,
            bbox: aabb_from_point(center.clone() - rvec.clone(), center + rvec),
        }
    }

    // Moving Sphere
    pub fn moving(center1: Vec3, center2: Vec3, radius: f64, mat: Material) -> Sphere {
        let rvec = vec3(radius, radius, radius);
        let center = ray(center1.clone(), center2 - center1);
        let box1 = &aabb_from_point(center.at(0.0) - rvec.clone(), center.at(0.0) + rvec.clone());
        let box2 = &aabb_from_point(center.at(1.0) - rvec.clone(), center.at(1.0) + rvec);
        Sphere {
            center,
            radius,
            mat,
            bbox: Aabb::from_aabb(box1, box2)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = &current_center - r.origin();
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
        let outward_normal = (&rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

/// Translate a point on the sphere two dimensional coordinates consisting on
/// horizontal and vertical angle
fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //      <1 0 0> yields <0.50 0.50>      <-1  0  0> yields <0.00 0.50>
    //      <0 1 0> yields <0.50 1.00>      < 0 -1  0> yields <0.50 0.00>
    //      <0 0 1> yields <0.25 0.50>      < 0  0 -1> yields <0.75 0.50>

    let theta = f64::acos(-p.y);
    let phi = f64::atan2(-p.z, p.x) + PI;

    *u = phi / (2.0 * PI);
    *v = theta / PI;
}