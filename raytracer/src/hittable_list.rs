// 可交互物体列表
// 物体需要支持 Hittable 的 trait

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closet_so_far = t_max;
        let mut hit_result: Option<HitRecord> = None;
        for object in self.objects.iter() {
            if let Some(hit_rec) = object.hit(ray, t_min, closet_so_far) {
                closet_so_far = hit_rec.t;
                hit_result = Some(hit_rec);
            }
        }
        hit_result
    }
}
