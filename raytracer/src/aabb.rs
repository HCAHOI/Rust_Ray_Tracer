use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, mut t_in: f64, mut t_out: f64) -> bool {
        for idx in 0..3 {
            let inv_d = 1.0 / r.direction().get(idx);
            let t0 = (self.min.get(idx) - r.origin().get(idx)) * inv_d;
            let t1 = (self.max.get(idx) - r.origin().get(idx)) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            (t_in, t_out) = (t_in.max(t0), t_out.min(t1));
            if t_out <= t_in {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let max = Vec3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );

    AABB { min, max }
}
