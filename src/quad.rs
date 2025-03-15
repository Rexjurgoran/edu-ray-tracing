use std::rc::Rc;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    sphere::{HitRecord, Hittable},
    vec3::{cross, dot, unit_vector, Vec3},
};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Material,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}
impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let mut quad = Quad {
            q,
            u,
            v,
            w: n / dot(&n, &n),
            mat,
            bbox: Default::default(),
            normal,
            d: dot(&normal, &q),
        };
        quad.set_bounding_box();
        quad
    }

    pub fn set_bounding_box(&mut self) {
        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = Aabb::from_point(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::from_point(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::from_aabb(&bbox_diagonal1, &bbox_diagonal2);
    }

    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::sphere::HitRecord,
    ) -> bool {
        let denom = dot(&self.normal, r.direction());

        // No hit if the ray is parallel to the plane.
        if f64::abs(denom) < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitp_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitp_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitp_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        true
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        &self.bbox
    }
}
