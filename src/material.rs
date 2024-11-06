use crate::{
    color::Color,
    ray::{ray, Ray},
    sphere::HitRecord,
    vec3::{dot, random_unit_vector, reflect, unit_vector},
};

#[derive(Clone, Default)]
pub enum Mat {
    #[default]
    Lambertian,
    Metal,
}

#[derive(Clone, Default)]
pub struct Material {
    material: Mat,
    albedo: Color,
    fuzz: f64,
}

pub fn material(material: Mat, albedo: Color, fuzz: f64) -> Material {
    Material {
        material,
        albedo,
        fuzz,
    }
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self.material {
            Mat::Lambertian => self.scatter_lambertian(r_in, rec, attenuation, scattered),
            Mat::Metal => self.scatter_metal(r_in, rec, attenuation, scattered),
        }
    }

    fn scatter_lambertian(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &random_unit_vector() + &rec.normal;
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *scattered = ray(rec.p.clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }

    fn scatter_metal(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = ray(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}
