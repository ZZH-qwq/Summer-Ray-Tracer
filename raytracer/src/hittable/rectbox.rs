// 长方体

use crate::aabb::Aabb;
use crate::hittable::aarect::*;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct RectBox {
    pub min: Vec3,
    pub max: Vec3,
    sides: HittableList,
}

impl RectBox {
    pub fn new<M: 'static + Material + Clone>(p0: Vec3, p1: Vec3, material: M) -> Self {
        let mut sides = HittableList::new();
        sides.add(Box::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.add(Box::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));
        sides.add(Box::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));
        sides.add(Box::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));
        sides.add(Box::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));
        sides.add(Box::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));
        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for RectBox {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
