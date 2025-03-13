use std::{i32, rc::Rc};

use bvh::BvhNode;
use camera::Camera;
use color::color;
use hittable_list::HittableList;
use material::Material;
use rtweekend::{random_double, random_double_from};
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture, NoiseTexture};
use vec3::{random, random_from, vec3, Vec3};

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

fn bouncing_spheres() {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));
    //let ground_material = Material::lambertian(color(0.5, 0.5, 0.5));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_from_tex(checker),
    )));

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
                    sphere_material = Material::lambertian(albedo.to_color());
                    let center2 = center.clone() + vec3(0.0, random_double_from(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_from(0.5, 1.0);
                    let fuzz = random_double_from(0.0, 0.5);
                    sphere_material = Material::metal(albedo.to_color(), fuzz);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Material::dielectric(rtweekend::REFRACTION_GLASS);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::dielectric(rtweekend::REFRACTION_GLASS);
    world.add(Rc::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Material::lambertian(color(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Material::metal(color(0.7, 0.6, 0.5), 0.0);
    world.add(Rc::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let node = BvhNode::from_list(&mut world);
    world = HittableList::new(Rc::new(node));

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

fn checkered_spheres() {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        color(0.2, 0.3, 0.1),
        color(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, -10.0, 0.0),
        10.0,
        Material::lambertian_from_tex(checker.clone()),
    )));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, 10.0, 0.0),
        10.0,
        Material::lambertian_from_tex(checker),
    )));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = vec3(13.0, 2.0, 3.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("misc\\earthmap.jpg"));
    let earth_surface = Material::lambertian_from_tex(earth_texture);
    let globe = Rc::new(Sphere::new(vec3(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = vec3(0.0, 0.0, 12.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new());
    world.add(Rc::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_from_tex(pertext.clone()),
    )));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, 2.0, 0.0),
        2.0,
        Material::lambertian_from_tex(pertext),
    )));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50;

    cam.vfov     = 20.0;
    cam.lookfrom = vec3(13.0,2.0,3.0);
    cam.lookat   = vec3(0.0,0.0,0.0);
    cam.vup      = vec3(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        i32::MIN..=i32::MAX => !panic!(),
    }
}
