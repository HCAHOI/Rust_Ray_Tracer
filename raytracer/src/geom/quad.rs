use crate::color::mat::Material;
use crate::hit::aabb::AABB;
use crate::hit::hit::{Hit, HitRecord};

use super::ray::Ray;
use super::vec3::Vec3;

pub enum Plane {
    XY,
    XZ,
    YZ,
}

pub struct Quad<M: Material> {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: M,
}

impl<M: Material> Quad<M> {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64, material: M) -> Quad<M> {
        Quad {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl<M: Material> Hit for Quad<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis_index, a_axis_index, b_axis_index) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::XZ => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };

        let t = (self.k - r.origin().get(k_axis_index)) / r.direction().get(k_axis_index);
        if t < t_min || t > t_max {
            None
        } else {
            let a = r.origin().get(a_axis_index) + t * r.direction().get(a_axis_index);
            let b = r.origin().get(b_axis_index) + t * r.direction().get(b_axis_index);
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = r.at(t);
                let mut normal = Vec3::new(0.0, 0.0, 0.0);
                normal.set(k_axis_index, 1.0);

                let mut rec = HitRecord {
                    position: p,
                    normal,
                    t,
                    u,
                    v,
                    front_face: false,
                    material: &self.material,
                };

                rec.set_face_normal(r, normal);

                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        // Pad to avoid NaNs
        let delta = 0.0001;
        let min = Vec3::new(self.a0, self.b0, self.k - delta);
        let max = Vec3::new(self.a1, self.b1, self.k + delta);

        Some(AABB::new(min, max))
    }
}
