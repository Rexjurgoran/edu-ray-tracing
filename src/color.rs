pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color{r, g, b}
}

pub fn write_color(color:Color){

    let ir = (255.999 * color.r) as i32;
    let ig = (255.999 * color.g) as i32;
    let ib = (255.999 * color.b) as i32;

    print!("{} {} {}\n", ir, ig, ib);
}

impl std::ops::Add<Color> for Color{
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color{
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Color{
           r: rhs * self.r,
           g: rhs * self.g,
           b: rhs * self.b 
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    
    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}