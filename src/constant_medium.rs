use std::rc::Rc;

use crate::{
    color::Color,
    interval::Interval,
    material::Material,
    rtweekend::random_double,
    sphere::{HitRecord, Hittable},
    texture::Texture,
    vec3,
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn from_tex(
        boundary: Rc<dyn Hittable>,
        density: f64,
        tex: Rc<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: (-1.0 / density),
            phase_function: Material::isotropic_from_tex(tex),
        }
    }

    pub fn new(boundary: Rc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: (-1.0 / density),
            phase_function: Material::isotropic(albedo),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::sphere::HitRecord,
    ) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }
        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2)
        {
            return false;
        }
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(random_double());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = vec3(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> &crate::aabb::Aabb {
        self.boundary.bounding_box()
    }
}
