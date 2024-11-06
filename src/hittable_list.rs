use crate::{interval::{Interval, interval}, ray::Ray, sphere::{HitRecord, Hittable}};

#[derive(Default)]
pub struct HittableList{
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList{
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let is_hit = object.hit(r, interval(ray_t.min, closest_so_far), &mut temp_rec);
            if is_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}