use crate::{
    color::Color,
    ray::{ray, Ray},
    sphere::HitRecord,
    vec3::{random_unit_vector, reflect},
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
}

pub fn material(material: Mat, albedo: Color) -> Material {
    Material { material, albedo }
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
        let reflected = reflect(r_in.direction(), &rec.normal);
        *scattered = ray(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();
        true
    }
}
