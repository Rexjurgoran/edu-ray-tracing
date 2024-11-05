use std::f64::INFINITY;

use hittable_list::HittableList;
use ray::{Ray, ray};
use vec3::{unit_vector, Vec3, vec3};
use color::{Color, color};
use sphere::{HitRecord, Hittable, Sphere};
use interval::interval;

mod rtweekend;
mod ray;
mod color;
mod vec3;
mod sphere;
mod hittable_list;
mod interval;

fn ray_color(r: Ray, world: &impl Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(&r, interval( 0.0, INFINITY), &mut rec){
        return 0.5 * (rec.normal.to_color() + color(1.0, 1.0, 1.0))
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color { r: 1.0, g: 1.0, b: 1.0 } + a * Color { r: 0.5, g: 0.7, b: 1.0 }
}

fn main() {   
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    let mut world= HittableList::default();
    world.add(Sphere{center: vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.add(Sphere{center: vec3(0.0, -100.5, -1.0), radius: 100.0});

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = vec3(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let viewport_v = Vec3{x: 0.0, y: -viewport_height, z: 0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = &viewport_u / image_width;
    let pixel_delta_v = &viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center - &vec3(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {

            let pixel_center = pixel00_loc.clone() + (i * &pixel_delta_u) + (j * &pixel_delta_v);
            let ray_direction = &pixel_center - &camera_center;
            let r = ray(&camera_center, ray_direction);

            let color = ray_color(r, &world);
            crate::color::write_color(color);
        }
    }
}
