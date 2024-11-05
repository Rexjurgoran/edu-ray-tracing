use hittable_list::HittableList;
use vec3::vec3;
use sphere::Sphere;
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

fn main() {   
    let mut world= HittableList::default();
    
    world.add(Sphere{center: vec3(0.0, 0.0, -1.0), radius: 0.5});
    world.add(Sphere{center: vec3(0.0, -100.5, -1.0), radius: 100.0});

    let mut cam: Camera = Default::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}
