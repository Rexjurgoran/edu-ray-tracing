use std::{i32, rc::Rc};

use bvh::BvhNode;
use camera::Camera;
use color::color;
use constant_medium::ConstantMedium;
use hittable_list::{HittableList, RotateY, Translate};
use material::Material;
use quad::{bx, Quad};
use rtweekend::{random_double, random_double_from};
use sphere::{Hittable, Sphere};
use texture::{CheckerTexture, ImageTexture, NoiseTexture};
use vec3::{random, random_from, vec3};

mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
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
    cam.background = color(0.70, 0.80, 1.00);

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
    cam.background = color(0.70, 0.80, 1.00);

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
    cam.background = color(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = vec3(0.0, 0.0, 12.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}

fn perlin_spheres() {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
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
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = color(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = vec3(13.0, 2.0, 3.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::default();

    // Materials
    let left_red = Material::lambertian(color(1.0, 0.2, 0.2));
    let back_green = Material::lambertian(color(0.2, 1.0, 0.2));
    let right_blue = Material::lambertian(color(0.2, 0.2, 1.0));
    let upper_orange = Material::lambertian(color(1.0, 0.5, 0.0));
    let lower_teal = Material::lambertian(color(0.2, 0.8, 0.8));

    // Quads
    world.add(Rc::new(Quad::new(
        vec3(-3.0, -2.0, 5.0),
        vec3(0.0, 0.0, -4.0),
        vec3(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        vec3(-2.0, -2.0, 0.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        vec3(3.0, -2.0, 1.0),
        vec3(0.0, 0.0, 4.0),
        vec3(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        vec3(-2.0, 3.0, 1.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        vec3(-2.0, -3.0, 5.0),
        vec3(4.0, 0.0, 0.0),
        vec3(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam: Camera = Default::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = color(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.lookfrom = vec3(0.0, 0.0, 9.0);
    cam.lookat = vec3(0.0, 0.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new(4.0));
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

    let difflight = Material::diffuse_light(color(4.0, 4.0, 4.0));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(Quad::new(
        vec3(3.0, 1.0, -2.0),
        vec3(2.0, 0.0, 0.0),
        vec3(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = color(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = vec3(26.0, 3.0, 6.0);
    cam.lookat = vec3(0.0, 2.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Material::lambertian(color(0.65, 0.05, 0.05));
    let white = Material::lambertian(color(0.73, 0.73, 0.73));
    let green = Material::lambertian(color(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(color(15.0, 15.0, 15.0));

    world.add(Rc::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        vec3(343.0, 554.0, 332.0),
        vec3(-130.0, 0.0, 0.0),
        vec3(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        vec3(555.0, 555.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = bx(
        &vec3(0.0, 0.0, 0.0),
        &vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, vec3(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = bx(&vec3(0.0, 0.0, 0.0), &vec3(165.0, 165.0, 165.0), white);
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, vec3(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = color(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = vec3(278.0, 278.0, -800.0);
    cam.lookat = vec3(278.0, 278.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::default();

    let red = Material::lambertian(color(0.65, 0.05, 0.05));
    let white = Material::lambertian(color(0.73, 0.73, 0.73));
    let green = Material::lambertian(color(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(color(7.0, 7.0, 7.0));

    world.add(Rc::new(Quad::new(
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        vec3(113.0, 554.0, 127.0),
        vec3(330.0, 0.0, 0.0),
        vec3(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 0.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        vec3(555.0, 555.0, 555.0),
        vec3(-555.0, 0.0, 0.0),
        vec3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        vec3(0.0, 0.0, 555.0),
        vec3(555.0, 0.0, 0.0),
        vec3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = bx(
        &vec3(0.0, 0.0, 0.0),
        &vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, vec3(265.0, 0.0, 295.0)));

    let mut box2: Rc<dyn Hittable> = bx(&vec3(0.0, 0.0, 0.0), &vec3(165.0, 165.0, 165.0), white);
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, vec3(130.0, 0.0, 65.0)));

    world.add(Rc::new(ConstantMedium::new(
        box1,
        0.01,
        color(0.0, 0.0, 0.0),
    )));
    world.add(Rc::new(ConstantMedium::new(
        box2,
        0.01,
        color(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = color(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = vec3(278.0, 278.0, -800.0);
    cam.lookat = vec3(278.0, 278.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 8 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        i32::MIN..=i32::MAX => !panic!(),
    }
}
