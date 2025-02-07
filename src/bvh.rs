use crate::{aabb::Aabb, interval::{interval, Interval}, ray::Ray, sphere::Hittable};

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: Aabb
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut crate::sphere::HitRecord) -> bool {
        if !self.bbox.hit(r.clone(), ray_t.clone()) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t.clone(), rec);
        let hit_right = self.right.hit(
            r, 
            interval(ray_t.min, if hit_left { rec.t } else { ray_t.max }), 
            rec
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}