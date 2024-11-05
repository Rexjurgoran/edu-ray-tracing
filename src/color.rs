use crate::interval::interval;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

pub fn write_color(color: Color) {
    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = interval(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(color.r)) as i32;
    let ig = (255.999 * intensity.clamp(color.g)) as i32;
    let ib = (255.999 * intensity.clamp(color.b)) as i32;

    // Write out the pixel color components.
    print!("{} {} {}\n", ir, ig, ib);
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: rhs * self.r,
            g: rhs * self.g,
            b: rhs * self.b,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
