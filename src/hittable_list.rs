use core::f64;
use std::{i32, rc::Rc};

use crate::{
    aabb::Aabb,
    interval::Interval,
    ray::{ray, ray_with_time, Ray},
    rtweekend::degrees_to_radians,
    sphere::{HitRecord, Hittable},
    vec3::{vec3, Vec3},
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,

    bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> HittableList {
        let mut hittable_list = HittableList {
            objects: Default::default(),
            bbox: Default::default(),
        };
        hittable_list.add(object);
        hittable_list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = Aabb::from_aabb(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let is_hit = object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec);
            if is_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = ray_with_time(r.origin() - self.offset, *r.direction(), r.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p = rec.p + self.offset;

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let mut bbox = object.bounding_box();

        let mut min = vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = vec3(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = vec3(new_x, y, new_z);

                    for c in 0..3 {
                        match c {
                            0 => {
                                min.x = f64::min(min.x, tester.x);
                                max.x = f64::min(max.x, tester.x)
                            }
                            1 => {
                                min.y = f64::min(min.y, tester.y);
                                max.y = f64::min(max.y, tester.y)
                            },
                            2 => {
                                min.z = f64::min(min.z, tester.z);
                                max.z = f64::min(max.z, tester.z)
                            }
                            i32::MIN..=i32::MAX => !panic!("Unreachable")
                        }
                    }
                }
            }
        }

        let binding = Aabb::from_point(&min, &max);
        bbox = &binding;

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: *bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.
        let origin = vec3(
            (self.cos_theta * r.origin().x) - (self.sin_theta * r.origin().z),
            r.origin().y,
            (self.sin_theta * r.origin().x) + (self.cos_theta * r.origin().z),
        );
        let direction = vec3(
            (self.cos_theta * r.direction().x) - (self.sin_theta * r.direction().z),
            r.direction().y,
            (self.sin_theta * r.direction().x) + (self.cos_theta * r.direction().z),
        );
        let rotated_r = &ray_with_time(origin, direction, r.time());

        // Determine whether an intersection exists in object space (and if so, where).
        if !self.object.hit(rotated_r, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space.
        rec.p = vec3(
            (self.cos_theta * rec.p.x) - (self.sin_theta * rec.p.z),
            rec.p.y,
            (-self.sin_theta * rec.p.x) + (self.cos_theta * rec.p.z),
        );
        rec.normal = vec3(
            (self.cos_theta * rec.normal.x) - (self.sin_theta * rec.normal.z),
            rec.normal.y,
            (-self.sin_theta * rec.normal.x) + (self.cos_theta * rec.normal.z),
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
