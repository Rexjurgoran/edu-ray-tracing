use camera::Camera;
use color::color;
use hittable_list::HittableList;
use interval::interval;
use material::{material_dielectric, material_lambertian, material_metal};
use rtweekend::{random_double, random_double_from};
use sphere::{sphere, sphere_moving};
use vec3::{random, random_from, vec3};

mod aabb;
mod camera;
mod color;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
mod bvh;

fn main() {
    let mut world = HittableList::default();

    // let r = f64::cos(PI / 4.0);

    // let material_left = material_lambertian(color(0.0, 0.0, 1.0));
    // let material_right = material_lambertian(color(1.0, 0.0, 0.0));

    // world.add(sphere(vec3(-r, 0.0, -1.0), r, material_left));
    // world.add(sphere(vec3(r, 0.0, -1.0), r, material_right));

    // let material_ground = material_lambertian(color(0.8, 0.8, 0.0));
    // let material_center = material_lambertian(color(0.1, 0.2, 0.5));
    // let material_left = material_dielectric(rtweekend::REFRACTION_GLASS);
    // let material_bubble = material_dielectric(rtweekend::REFRACTION_AIR/rtweekend::REFRACTION_GLASS);
    // let material_right = material_metal(color(0.8, 0.6, 0.2), 1.0);

    // world.add(sphere(vec3(0.0, -100.5, -1.0), 100.0, material_ground));
    // world.add(sphere(vec3(0.0, 0.0, -1.2), 0.5, material_center));
    // world.add(sphere(vec3(-1.0, 0.0, -1.0), 0.5, material_left));
    // world.add(sphere(vec3(-1.0, 0.0, -1.0), 0.4, material_bubble));
    // world.add(sphere(vec3(1.0, 0.0, -1.0), 0.5, material_right));

    let ground_material = material_lambertian(color(0.5, 0.5, 0.5));
    world.add(sphere(vec3(0.0, -1000.0, 0.0), -1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = vec3(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            let sphere_material;

            if (&center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random() * random();
                    sphere_material = material_lambertian(albedo.to_color());
                    let center2 = center.clone() + vec3(0.0, random_double_from(0.0, 0.5), 0.0);
                    world.add(sphere_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_from(0.5, 1.0);
                    let fuzz = random_double_from(0.0, 0.5);
                    sphere_material = material_metal(albedo.to_color(), fuzz);
                    world.add(sphere(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = material_dielectric(rtweekend::REFRACTION_GLASS);
                    world.add(sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = material_dielectric(rtweekend::REFRACTION_GLASS);
    world.add(sphere(vec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = material_lambertian(color(0.4, 0.2, 0.1));
    world.add(sphere(vec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = material_metal(color(0.7, 0.6, 0.5), 0.0);
    world.add(sphere(vec3(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = vec3(13.0, 2.0, 3.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
