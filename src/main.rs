use color::color;
use hittable_list::HittableList;
use material::{material, Mat};
use vec3::vec3;
use sphere::sphere;
use interval::interval;
use camera::Camera;

mod rtweekend;
mod ray;
mod color;
mod vec3;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

fn main() {   
    let mut world= HittableList::default();

    let material_ground = material(Mat::Lambertian,color(0.8, 0.8, 0.0), 0.0);
    let material_center = material(Mat::Lambertian,color(0.1, 0.2, 0.5), 0.0);
    let material_left = material(Mat::Metal,color(0.8, 0.8, 0.8), 0.3);
    let material_right = material(Mat::Metal,color(0.8, 0.6, 0.2), 1.0);
    
    world.add(sphere(vec3(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(sphere(vec3(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(sphere(vec3(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(sphere(vec3(1.0, 0.0, -1.0), 0.5, material_right));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
