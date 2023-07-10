// 纹理

pub mod perlin;
use crate::vec3::{Color, Vec3};
use image::{GenericImageView, ImageBuffer, Rgb};
use perlin::Perlin;
use std::path::Path;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

// 纯色
#[derive(Clone, Copy)]
pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Vec3) -> Color {
        self.color_value
    }
}

// 棋盘格
#[derive(Clone, Copy)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    pub odd: T1,
    pub even: T2,
}

impl<T1: Texture, T2: Texture> CheckerTexture<T1, T2> {
    pub fn new(even: T2, odd: T1) -> Self {
        Self { odd, even }
    }
}

impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

// 噪声
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: Vec3) -> Color {
        Color::one() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.trub(p, 7)).sin())
    }
}

// 贴图
pub struct ImageTexture {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(file_name: String) -> Self {
        let im = image::open(&Path::new(&file_name)).unwrap();
        let dim = im.dimensions();
        let data = im.into_rgb8();
        Self {
            data,
            width: dim.0,
            height: dim.1,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Color {
        let i = ((u * self.width as f64) as u32).clamp(0, self.width - 1);
        let j = ((v * self.height as f64) as u32).clamp(0, self.height - 1);
        let rgb = self.data[(i, j)].0;
        Color::new(
            (rgb[0] as f64) / 255.0,
            (rgb[1] as f64) / 255.0,
            (rgb[2] as f64) / 255.0,
        )
    }
}
