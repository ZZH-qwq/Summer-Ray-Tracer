// 旋转

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Translate {
    pub object: Box<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        self.object.hit(moved_r, t_min, t_max).map(|hit_record| {
            HitRecord::new(
                hit_record.point + self.offset,
                hit_record.t,
                hit_record.u,
                hit_record.v,
                hit_record.normal,
                hit_record.material,
                moved_r,
            )
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.object.bounding_box(time0, time1).map(|bounding_box| {
            Aabb::new(
                bounding_box.min + self.offset,
                bounding_box.max + self.offset,
            )
        })
    }
}

pub struct RotateY {
    pub object: Box<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(object: Box<dyn Hittable>, theta: f64) -> Self {
        let radians = theta.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box(0.0, 1.0).map(|bbox| {
            let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max.x + (1 - i) as f64 * bbox.min.x;
                        let y = j as f64 * bbox.max.y + (1 - j) as f64 * bbox.min.y;
                        let z = k as f64 * bbox.max.z + (1 - k) as f64 * bbox.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Aabb::new(min, max)
        });
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_r = Ray::new(origin, direction, ray.time);

        self.object.hit(rotated_r, t_min, t_max).map(|hit_record| {
            let mut p = hit_record.point;
            let mut normal = hit_record.normal;

            p[0] = self.cos_theta * hit_record.point[0] + self.sin_theta * hit_record.point[2];
            p[2] = -self.sin_theta * hit_record.point[0] + self.cos_theta * hit_record.point[2];

            normal[0] =
                self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
            normal[2] =
                -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

            HitRecord::new(
                p,
                hit_record.t,
                hit_record.u,
                hit_record.v,
                normal,
                hit_record.material,
                rotated_r,
            )
        })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        self.bbox
    }
}
