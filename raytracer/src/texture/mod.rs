// 纹理

pub mod perlin;
use crate::vec3::{Color, Vec3};
use image::{GenericImageView, ImageBuffer, Rgb};
use perlin::Perlin;
use radiant::RGB;
use std::io::BufReader;
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
        // Color::one() * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.trub(p, 7)).sin())
        Color::one() * self.noise.trub(self.scale * p, 7)
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

// HDRi图像
pub struct HdrImageTexture {
    data: Vec<RGB>,
    width: u32,
    height: u32,
    total: usize,
    limit: f64,
}

impl HdrImageTexture {
    pub fn new(file_name: String, limit: f64) -> Self {
        let f = std::fs::File::open(&file_name).expect("Failed to open specified file");
        let f = BufReader::new(f);
        let image = radiant::load(f).expect("Failed to load image data");
        let data = image.data;
        Self {
            data,
            width: image.width as u32,
            height: image.height as u32,
            total: image.width * image.height,
            limit,
        }
    }
}

impl Texture for HdrImageTexture {
    fn value(&self, u: f64, v: f64, _: Vec3) -> Color {
        let ii = u * self.width as f64;
        let jj = v * self.height as f64;
        let di = ii.fract() as f32;
        let dj = jj.fract() as f32;
        let v00 = (1.0 - di) * (1.0 - dj);
        let v10 = di * (1.0 - dj);
        let v01 = (1.0 - di) * dj;
        let v11 = di * dj;
        let i = (ii as u32).clamp(0, self.width - 1);
        let j = (jj as u32).clamp(0, self.height - 1);
        let idx = (j * self.width + i) as usize;
        let rgb00 = &self.data[idx];
        let rgb10 = &self.data[(idx + 1).clamp(0, self.total)];
        let rgb01 = &self.data[(idx + self.width as usize).clamp(0, self.total)];
        let rgb11 = &self.data[(idx + self.width as usize + 1).clamp(0, self.total)];
        Color::new(
            (rgb00.r * v00 + rgb01.r * v01 + rgb10.r * v10 + rgb11.r * v11) as f64,
            (rgb00.g * v00 + rgb01.g * v01 + rgb10.g * v10 + rgb11.g * v11) as f64,
            (rgb00.b * v00 + rgb01.b * v01 + rgb10.b * v10 + rgb11.b * v11) as f64,
        ) * self.limit
    }
}
