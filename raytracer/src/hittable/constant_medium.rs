// 恒密度介质

use crate::hittable::*;
use crate::material::Material;
use rand::Rng;

pub struct ConstantMedium<M: Material> {
    pub boundary: Box<dyn Hittable>,
    pub material: M,
    pub neg_inv_density: f64,
}

impl<M: Material> ConstantMedium<M> {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, material: M) -> Self {
        Self {
            boundary,
            material,
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<M: Material> Hittable for ConstantMedium<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY) {
            Some(hit_record) => hit_record,
            None => return None,
        };
        let mut rec2 = match self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY) {
            Some(hit_record) => hit_record,
            None => return None,
        };
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::thread_rng().gen::<f64>().log2();
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let t = rec1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            ray.at(t),
            t,
            0.0,
            0.0,
            Vec3::new(1.0, 0.0, 0.0),
            &self.material,
            ray,
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
