use crate::{
    color::{color, Color},
    ray::{ray, Ray},
    rtweekend::random_double,
    sphere::HitRecord,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3},
};

#[derive(Clone, Default)]
pub enum Mat {
    #[default]
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone, Default)]
pub struct Material {
    material: Mat,
    albedo: Color,
    fuzz: f64,
    refraction_index: f64,
}

pub fn material(material: Mat, albedo: Color, fuzz: f64, refraction_index: f64) -> Material {
    Material {
        material,
        albedo,
        fuzz,
        refraction_index,
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
            Mat::Dielectric => self.scatter_dielectic(r_in, rec, attenuation, scattered),
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

    fn scatter_dielectic(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = color(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(&-&unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        *scattered = ray(rec.p.clone(), direction);
        true
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}
