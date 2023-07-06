use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<(Color, Ray)>;
}

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
    fn scatter(&self, _: Ray, hit_record: HitRecord) -> Option<(Color, Ray)> {
        let direction = hit_record.normal + Vec3::random_unit_vector();
        if direction.near_zero() {
            Some((self.albedo, Ray::new(hit_record.point, hit_record.normal)))
        } else {
            Some((self.albedo, Ray::new(hit_record.point, direction)))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(Vec3::unit_vector(ray.direction), hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit_record.point, reflected)))
        } else {
            None
        }
    }
}
