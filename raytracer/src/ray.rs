// 光线类

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    // at() 能够求出光路上的某一点
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
