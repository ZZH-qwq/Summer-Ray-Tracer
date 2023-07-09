// BVH节点类

use crate::aabb::Aabb;
use crate::hittable::*;
use crate::hittable_list::HittableList;
use rand::Rng;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bounding_box: Aabb,
}

impl BVHNode {
    fn create_tree(
        mut objects: Vec<Box<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Box<dyn Hittable> {
        match objects.len() {
            0 => panic!("[BVH] len mismatch"),
            1 => objects.remove(0),
            _ => {
                let mut rng = rand::thread_rng();
                let axis = rng.gen_range(0..3);
                objects.sort_by(|a, b| {
                    a.bounding_box(time0, time1).unwrap().min[axis]
                        .partial_cmp(&b.bounding_box(time0, time1).unwrap().min[axis])
                        .unwrap()
                });

                let mut a = objects;
                let b = a.split_off(a.len() / 2);
                let left = Self::create_tree(a, time0, time1);
                let right = Self::create_tree(b, time0, time1);
                let bounding_box = Aabb::surrounding_box(
                    &left.bounding_box(time0, time1).unwrap(),
                    &right.bounding_box(time0, time1).unwrap(),
                );
                Box::new(Self {
                    left,
                    right,
                    bounding_box,
                })
            }
        }
    }

    pub fn create(hittable_list: HittableList, time0: f64, time1: f64) -> Box<dyn Hittable> {
        Self::create_tree(hittable_list.objects, time0, time1)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);
        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(hit_record), None) => Some(hit_record),
            (None, Some(hit_record)) => Some(hit_record),
            (Some(record_left), Some(record_right)) => {
                if record_left.t < record_right.t {
                    Some(record_left)
                } else {
                    Some(record_right)
                }
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
