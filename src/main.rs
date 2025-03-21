use std::{i32, rc::Rc};

use bvh::BvhNode;
use camera::Camera;
use color::Color;
use constant_medium::ConstantMedium;
use hittable_list::{HittableList, RotateY, Translate};
use material::Material;
use quad::{bx, Quad};
use rtweekend::{random_double, random_double_from, REFRACTION_GLASS};
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
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    //let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
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

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Rc::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let node = BvhNode::from_list(&mut world);
    world = HittableList::new(Rc::new(node));

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

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
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
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
    cam.background = Color::new(0.70, 0.80, 1.00);

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
    cam.background = Color::new(0.70, 0.80, 1.00);

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
    cam.background = Color::new(0.70, 0.80, 1.00);

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
    let left_red = Material::lambertian(Color::new(1.0, 0.2, 0.2));
    let back_green = Material::lambertian(Color::new(0.2, 1.0, 0.2));
    let right_blue = Material::lambertian(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Material::lambertian(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Material::lambertian(Color::new(0.2, 0.8, 0.8));

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
    cam.background = Color::new(0.70, 0.80, 1.00);

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

    let difflight = Material::diffuse_light(Color::new(4.0, 4.0, 4.0));
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
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = vec3(26.0, 3.0, 6.0);
    cam.lookat = vec3(0.0, 2.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::default();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(15.0, 15.0, 15.0));

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
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = vec3(278.0, 278.0, -800.0);
    cam.lookat = vec3(278.0, 278.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::default();

    let red = Material::lambertian(Color::new(0.65, 0.05, 0.05));
    let white = Material::lambertian(Color::new(0.73, 0.73, 0.73));
    let green = Material::lambertian(Color::new(0.12, 0.45, 0.15));
    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));

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
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Rc::new(ConstantMedium::new(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 800;
    cam.samples_per_pixel = 10000;
    cam.max_depth = 40;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = vec3(278.0, 278.0, -800.0);
    cam.lookat = vec3(278.0, 278.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn final_scene(image_width: i32, samples_per_pixel: i32, max_depth: i32) {
    // Create a grid of 20x20 boxes of different height as floor
    let mut boxes1 = HittableList::default();
    let ground = Material::lambertian(Color::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0; // width of boxes
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_from(1.0, 101.0); // give boxes a random heigth
            let z1 = z0 + w;

            boxes1.add(bx(&vec3(x0, y0, z0), &vec3(x1, y1, z1), ground.clone()));
        }
    }

    let mut world = HittableList::default();

    world.add(Rc::new(BvhNode::from_list(&mut boxes1)));

    // Create a rectangular light source above all other components
    let light = Material::diffuse_light(Color::new(7.0, 7.0, 7.0));
    world.add(Rc::new(Quad::new(
        vec3(123.0, 554.0, 147.0),
        vec3(300.0, 0.0, 0.0),
        vec3(0.0, 0.0, 265.0),
        light,
    )));

    // Create a moving sphere
    let center1 = vec3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);

    let sphere_material = Material::lambertian(Color::new(0.7, 0.3, 0.1));
    world.add(Rc::new(Sphere::moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // Create a stationary glass and a stationary metal sphere
    world.add(Rc::new(Sphere::new(
        vec3(260.0, 150.0, 45.0),
        50.0,
        Material::dielectric(REFRACTION_GLASS),
    )));
    world.add(Rc::new(Sphere::new(
        vec3(0.0, 150.0, 145.0),
        50.0,
        Material::metal(Color::new(0.8, 0.8, 0.9), 1.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.lookfrom = vec3(478.0, 278.0, -600.0);
    cam.lookat = vec3(278.0, 278.0, 0.0);
    cam.vup = vec3(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 10 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        i32::MIN..=i32::MAX => final_scene(400, 250, 4),
    }
}
