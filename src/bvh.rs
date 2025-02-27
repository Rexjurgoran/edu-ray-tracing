use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::Aabb, hittable_list::HittableList, interval::Interval, ray::Ray, rtweekend::random_int_from, sphere::{HitRecord, Hittable}
};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        // Build the bounding box of the span of source objects.
        // let mut bbox = Aabb::empty();

        // Optimization that just don't seem to work
        // for object_index in start..end {
        //     bbox = Aabb::from_aabb(&bbox, objects[object_index].bounding_box());
        // }

        let object_span = end - start;
        let left;
        let right;
        
        // Part of optimization code
        // let axis = bbox.longest_axis();
        let axis = random_int_from(0, 2);

        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects.sort_by(comparator);

            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::new(objects, start, mid));
            right = Rc::new(BvhNode::new(objects, mid, end));
        }
        let bbox = Aabb::from_aabb(left.bounding_box(), right.bounding_box());
        BvhNode { left, right, bbox }
    }

    /// Constructs a BVH node from a hittable list instance
    pub fn from_list(list: &mut HittableList) -> BvhNode {
        let end = list.objects.len();
        BvhNode::new(&mut list.objects, 0, end)
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r.clone(), ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
