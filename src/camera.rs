use std::f64::INFINITY;

use crate::{
    color::{color, write_color, Color},
    interval,
    ray::{ray, Ray},
    rtweekend::random_double,
    sphere::{HitRecord, Hittable},
    vec3::{unit_vector, vec3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene

    image_height: i32,       // Rendered image height
    pixel_sample_scale: f64, // Color scale factor for a sum of pixel samples
    center: Vec3,            // Camera center
    pixel00_loc: Vec3,       // Location of pixel 0, 0
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel below
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            pixel_sample_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        Camera::initialize(self);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = color(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world)
                }
                write_color(self.pixel_sample_scale * pixel_color);
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = vec3(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v = Vec3 {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width;
        self.pixel_delta_v = &viewport_v / self.image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &self.center - &vec3(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + ((i as f64 + offset.x) * &self.pixel_delta_u)
            + ((j as f64 + offset.y) * &self.pixel_delta_v);

        let ray_origin = &self.center;
        let ray_direction = &pixel_sample - ray_origin;

        ray(ray_origin.clone(), ray_direction)
    }
}

fn sample_square() -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
}

fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();

    if world.hit(&r, interval(0.001, INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return color(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)
        * Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
        + a * Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        }
}
