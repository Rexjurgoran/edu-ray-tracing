use core::f64;

use crate::{
    color::{Color, write_color},
    interval::Interval,
    ray::{ray_with_time, Ray},
    rtweekend::{degrees_to_radians, random_double},
    sphere::{HitRecord, Hittable},
    vec3::{cross, random_in_unit_disk, unit_vector, vec3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene
    pub background: Color,      // Scene background color

    pub vfov: f64,      // Vertical view angle (field of view)
    pub lookfrom: Vec3, // Point camera is looking from
    pub lookat: Vec3,   // Point camera is looking at
    pub vup: Vec3,      // Camera-relative "up" direction

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: i32,       // Rendered image height
    pixel_sample_scale: f64, // Color scale factor for a sum of pixel samples
    center: Vec3,            // Camera center
    pixel00_loc: Vec3,       // Location of pixel 0, 0
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel below
    u: Vec3,                 // Camera frame basis vectors
    v: Vec3,                 //
    w: Vec3,                 //
    defocus_disk_u: Vec3,    // Defocus disk horizontal radius
    defocus_disk_v: Vec3,    // Defocus disk vertical radius
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: vec3(0.0, 0.0, 0.0),
            lookat: vec3(0.0, 0.0, -1.0),
            vup: vec3(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: Default::default(),
            pixel_sample_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
            background: Color::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        Camera::initialize(self);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world)
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

        self.center = self.lookfrom.clone();

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame.
        self.w = unit_vector(&(&self.lookfrom - &self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * &self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -&self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / self.image_width;
        self.pixel_delta_v = &viewport_v / self.image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &self.center - (self.focus_dist * &self.w) - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + ((i as f64 + offset.x) * &self.pixel_delta_u)
            + ((j as f64 + offset.y) * &self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;
        let ray_time = random_double();

        ray_with_time(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        // Returns a random point in the camera defocus disk
        let p = random_in_unit_disk();
        &self.center + (p.x * &self.defocus_disk_u) + (p.y * &self.defocus_disk_v)
    }

    pub fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // If the ray hits nothing, return the background color.
        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emmission = rec.mat.emmited(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emmission;
        }

        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);

        color_from_emmission + color_from_scatter
    }
}

fn sample_square() -> Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
}
