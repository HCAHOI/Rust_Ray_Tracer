use crate::geom::vec3::Vec3;

use super::{color::Color, perlin::Perlin};

pub trait Texture: Sync {
    fn texture_map(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Copy, Clone)]
pub struct ConstantTexture {
    value: Color,
}

impl ConstantTexture {
    pub fn new(color: Color) -> ConstantTexture {
        ConstantTexture { value: color }
    }
}

impl Texture for ConstantTexture {
    fn texture_map(&self, _: f64, _: f64, _: &Vec3) -> Color {
        self.value
    }
}

#[derive(Copy, Clone)]
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

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> ImageTexture {
        ImageTexture {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn texture_map(&self, u: f64, v: f64, _: &Vec3) -> Color {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let mut i = (u.clamp(0.0, 1.0) * self.width as f64) as usize;
        // Flip V to image coordinates
        let mut j = ((1.0 - v).clamp(0.0, 1.0) * self.height as f64) as usize;
        // Clamp integer mapping, since actual coordinates should be less than 1.0
        let w = self.width as usize;
        let h = self.height as usize;
        if i > w - 1 {
            i = w - 1
        }
        if j > h - 1 {
            j = h - 1
        }
        //3 bytes per pixel
        let idx = 3 * i + 3 * w * j;
        Color::new(
            self.data[idx] as f64,
            self.data[idx + 1] as f64,
            self.data[idx + 2] as f64,
        ) / 255.0
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn texture_map(&self, _u: f64, _v: f64, p: &Vec3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, self.scale, 7)))
    }
}
