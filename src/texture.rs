use std::rc::Rc;

use crate::{color::{color, Color}, vec3::Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn from_color(albedo: Color) -> SolidColor {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        Self { albedo: color(red, green, blue) }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { inv_scale: 1.0 / scale, even, odd }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture::new(
            scale, 
            Rc::new(SolidColor::from_color(c1)), 
            Rc::new(SolidColor::from_color(c2))
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x = f64::floor(self.inv_scale * p.x) as i32;
        let y = f64::floor(self.inv_scale * p.y) as i32;
        let z = f64::floor(self.inv_scale * p.z) as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even { self.even.value(u, v, p) } else { self.odd.value(u, v, p) }
    }
}

