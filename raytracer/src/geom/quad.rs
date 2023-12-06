use rand::Rng;

use crate::hit::aabb::AABB;
use crate::hit::hittable::{HitRecord, Hittable};
use crate::render::mat::Material;

use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone)]
pub enum Plane {
    XY,
    XZ,
    YZ,
}

impl Plane {
    fn get_axis_index(&self) -> (usize, usize, usize) {
        match self {
            Plane::YZ => (0, 1, 2),
            Plane::XZ => (1, 0, 2),
            Plane::XY => (2, 0, 1),
        }
    }
}

#[derive(Clone)]
pub struct Quad<M: Material> {
    plane: Plane, // give the plane of the quad
    a0: f64,      // start point of the quad on the first axis
    a1: f64,      // end point of the quad on the first axis
    b0: f64,
    b1: f64,
    k: f64, // position of the quad on the remaining axis(XY plane, k = z)
    material: M,
}

impl<M: Material> Quad<M> {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64, material: M) -> Self {
        Self {
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

impl<M: Material> Hittable for Quad<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis_index, a_axis_index, b_axis_index) = self.plane.get_axis_index();

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
        let delta = 1e-4;
        let min = Vec3::new(self.a0, self.b0, self.k - delta);
        let max = Vec3::new(self.a1, self.b1, self.k + delta);

        Some(AABB::new(min, max))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, f64::INFINITY) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = rec.t.powi(2) * v.length().powi(2);
            let cos = v.dot(rec.normal).abs() / v.length();
            if cos != 0.0 {
                distance_squared / (cos * area)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let (k_axis, a_axis, b_axis) = self.plane.get_axis_index();
        let mut random_point = Vec3::zero();
        random_point.set(a_axis, rng.gen_range(self.a0..self.a1));
        random_point.set(b_axis, rng.gen_range(self.b0..self.b1));
        random_point.set(k_axis, self.k);
        random_point - o
    }
}
