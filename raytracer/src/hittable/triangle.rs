// 三角形面

use crate::aabb::Aabb;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Triangle<M: Material> {
    pub a: Vec3,
    pub n: Vec3,
    pub pc: Vec3,
    pub pb: Vec3,
    pub bbox: Aabb,
    pub material: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: M) -> Self {
        let e1 = b - a;
        let e2 = c - a;
        let ne = Vec3::cross(e1, e2);
        let nel = ne.length();
        let n = Vec3::unit_vector(ne);

        let mut min = Vec3::zero();
        let mut max = Vec3::zero();
        for i in 0..3 {
            min[i] = a[i].min(b[i]).min(c[i]);
            max[i] = a[i].max(b[i]).max(c[i]);
        }

        Self {
            a,
            n,
            pc: Vec3::cross(e2, n) / nel,
            pb: Vec3::cross(n, e1) / nel,
            bbox: Aabb::new(min, max),
            material,
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oa = ray.origin - self.a;
        let t = -Vec3::dot(oa, self.n) / Vec3::dot(ray.direction, self.n);
        if t < t_min || t > t_max {
            None
        } else {
            let p = oa + t * ray.direction;
            let u = Vec3::dot(self.pc, p);
            let v = Vec3::dot(self.pb, p);
            if (0.0..=1.0).contains(&(u + v))
                && (0.0..=1.0).contains(&u)
                && (0.0..=1.0).contains(&v)
            {
                Some(HitRecord::new(
                    ray.at(t),
                    t,
                    u,
                    v,
                    self.n,
                    &self.material,
                    ray,
                ))
            } else {
                None
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
