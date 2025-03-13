use std::rc::Rc;

use crate::{aabb::Aabb, material::Material, sphere::Hittable, vec3::{cross, dot, unit_vector, Vec3}};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Material,
    bbox: Aabb,
    normal: Vec3,
    d: f64
}
impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Material) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let mut quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: Default::default(),
            normal,
            d: dot(&normal, &q)
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
        if f64::abs(denom) < 1e-8 { return false; }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) { return false; }

        let intersection = r.at(t);

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
