// 材质

use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::Rng;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

// 漫反射
#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let direction = hit_record.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            Some((self.albedo, Ray::new(hit_record.point, hit_record.normal)))
        } else {
            Some((self.albedo, Ray::new(hit_record.point, direction)))
        }
    }
}

// 金属
#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray.direction), hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            Some((
                self.albedo,
                Ray::new(
                    hit_record.point,
                    reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                ),
            ))
        } else {
            None
        }
    }
}

// 折射
#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    // 非全反射时 折射存在概率
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // 施里克近似
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0p = r0 * r0;
        r0p + (1.0 - r0p) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(ray.direction);
        let cos_theta = Vec3::dot(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_reflact = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let rand_double: f64 = rng.gen();
        if cannot_reflact || Dielectric::reflectance(cos_theta, refraction_ratio) > rand_double {
            // 反射
            let reflected = Vec3::reflect(unit_direction, hit_record.normal);
            Some((
                Color::new(1.0, 1.0, 1.0),
                Ray::new(hit_record.point, reflected),
            ))
        } else {
            // 折射
            let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);
            Some((
                Color::new(1.0, 1.0, 1.0),
                Ray::new(hit_record.point, refracted),
            ))
        }
    }
}
