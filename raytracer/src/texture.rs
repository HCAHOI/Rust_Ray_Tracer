use crate::color::Color;
use crate::vec3::Vec3;

pub trait Texture: Sync {
    fn texture_map(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct ConstantTexture {
    value: Color,
}

impl ConstantTexture {
    pub fn new(color: Color) -> ConstantTexture {
        ConstantTexture { value: color }
    }
}

impl Texture for ConstantTexture {
    fn texture_map(&self, u: f64, v: f64, p: &Vec3) -> Color {
        self.value
    }
}

pub struct CheckerTexture<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(odd: T, even: U) -> CheckerTexture<T, U> {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn texture_map(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.texture_map(u, v, p)
        } else {
            self.even.texture_map(u, v, p)
        }
    }
}
