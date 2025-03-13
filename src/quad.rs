use std::rc::Rc;

use crate::{aabb::Aabb, material::Material, sphere::Hittable, vec3::Vec3};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Rc<Material>,
    bbox: Aabb,
}
impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Rc<Material>) -> Self {
        let mut quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: Default::default(),
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
        todo!()
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        &self.bbox
    }
}
