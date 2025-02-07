use crate::{aabb::{aabb_from_aabb, Aabb}, interval::{interval, Interval}, ray::Ray, sphere::Hittable};

pub struct BvhNode {
    left: usize,
    right: usize,
    bbox: Aabb,
}

static NODES:Vec<BvhNode> = Vec::new();

pub fn bvh_node(objects: &Vec<Box<dyn Hittable>>, nodes: &mut Vec<BvhNode>, start: usize, end: usize) -> usize {
    let object_span = end - start;
    let mut left = start ;
    let mut right = start;

    if object_span == 1 {
        // Keep initial values for left and right
    } else if object_span == 2 {
        right = start + 1;
    } else {

        // TODO: sorting of objects by random axis

        let mid = start + object_span / 2;

        left = bvh_node(objects, nodes, start, mid);
        right = bvh_node(objects, nodes, mid, end);
    }

    let bbox = aabb_from_aabb(nodes[left].bounding_box(), nodes[right].bounding_box());
    let node = BvhNode { left, right, bbox };
    nodes.push(node);
    nodes.len()
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut crate::sphere::HitRecord) -> bool {
        if !self.bbox.hit(r.clone(), ray_t.clone()) {
            return false;
        }

        let hit_left = NODES[self.left].hit(r, ray_t.clone(), rec);
        let hit_right = NODES[self.right].hit(
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