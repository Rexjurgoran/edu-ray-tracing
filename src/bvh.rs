use std::{cmp::Ordering, rc::Rc};

use crate::{aabb::{aabb_from_aabb, Aabb}, hittable_list::HittableList, interval::{interval, Interval}, ray::Ray, rtweekend::random_int_from, sphere::Hittable};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

pub fn bvh_node_from_list(hittable_list: &mut HittableList, nodes: &mut Vec<BvhNode>) -> BvhNode {
    let end = hittable_list.objects.len();
    bvh_node(&mut hittable_list.objects, nodes, 0, end)
}

pub fn bvh_node(objects: &mut Vec<Rc<dyn Hittable>>, nodes: &mut Vec<BvhNode>, start: usize, end: usize) -> BvhNode {
    let object_span = end - start;
    let mut left = objects[start].clone();
    let mut right = objects[start].clone();
    let axis = random_int_from(0, 2);

    let comparator = if axis == 0 {
        box_x_compare
    } else if axis == 1 {
        box_y_compare
    } else {
        box_z_compare
    };

    if object_span == 1 {
        // Keep initial values for left and right
    } else if object_span == 2 {
        right = objects[start + 1].clone();
    } else {

        objects.sort_by(comparator);

        let mid = start + object_span / 2;

        left = Rc::new(bvh_node(objects, nodes, start, mid));
        right = Rc::new(bvh_node(objects, nodes, mid, end));
    }

    let bbox = aabb_from_aabb(left.bounding_box(), right.bounding_box());
    BvhNode{left, right, bbox}
}

pub fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: i32) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
}

pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
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