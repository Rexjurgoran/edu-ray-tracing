use std::rc::Rc;

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::{color::Color, interval::Interval, perlin::Perlin, vec3::Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn from_color(albedo: Color) -> SolidColor {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        Self {
            albedo: Color::new(red, green, blue),
        }
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
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture::new(
            scale,
            Rc::new(SolidColor::from_color(c1)),
            Rc::new(SolidColor::from_color(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x = f64::floor(self.inv_scale * p.x) as i32;
        let y = f64::floor(self.inv_scale * p.y) as i32;
        let z = f64::floor(self.inv_scale * p.z) as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = match ImageReader::open(filename) {
            Ok(image_reader) => match image_reader.decode() {
                Ok(image) => image,
                Err(error) => panic!("Error decoding image reader: {error:?}"),
            },
            Err(error) => panic!("Error opening the image: {error:?}"),
        };
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (self.image.width() as f64 * u) as u32;
        let j = (self.image.height() as f64 * v) as u32;
        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, 7)))
    }
}
