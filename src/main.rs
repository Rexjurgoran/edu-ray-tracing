use std::{f64::consts::PI, vec};

use camera::Camera;
use color::color;
use hittable_list::HittableList;
use interval::interval;
use material::{material_dielectric, material_lambertian, material_metal};
use sphere::sphere;
use vec3::vec3;

mod camera;
mod color;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world = HittableList::default();

    // let r = f64::cos(PI / 4.0);

    // let material_left = material_lambertian(color(0.0, 0.0, 1.0));
    // let material_right = material_lambertian(color(1.0, 0.0, 0.0));

    // world.add(sphere(vec3(-r, 0.0, -1.0), r, material_left));
    // world.add(sphere(vec3(r, 0.0, -1.0), r, material_right));

    let material_ground = material_lambertian(color(0.8, 0.8, 0.0));
    let material_center = material_lambertian(color(0.1, 0.2, 0.5));
    let material_left = material_dielectric(rtweekend::REFRACTION_GLASS);
    let material_bubble = material_dielectric(rtweekend::REFRACTION_AIR/rtweekend::REFRACTION_GLASS);
    let material_right = material_metal(color(0.8, 0.6, 0.2), 1.0);

    world.add(sphere(vec3(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(sphere(vec3(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(sphere(vec3(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(sphere(vec3(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(sphere(vec3(1.0, 0.0, -1.0), 0.5, material_right));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = vec3(-2.0, 2.0, 1.0);
    cam.lookat = vec3(0.0, 0.0, -1.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.render(&world);
}
