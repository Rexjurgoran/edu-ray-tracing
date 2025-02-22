use crate::interval::Interval;

#[derive(Clone, Default, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }
    0.0
}

pub fn write_color(color: Color) {
    let mut r = color.r;
    let mut g = color.g;
    let mut b = color.b;

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

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

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        color(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
