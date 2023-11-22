#![allow(dead_code)]
use crate::{
    geom::{ray::Ray, vec3::Vec3},
    hit::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
    },
};

pub enum Axis {
    X,
    Y,
    Z,
}

fn get_axis_index(axis: &Axis) -> (usize, usize, usize) {
    match axis {
        Axis::X => (0, 1, 2),
        Axis::Y => (1, 0, 2),
        Axis::Z => (2, 0, 1),
    }
}

pub struct Rotate<H: Hittable> {
    axis: Axis,
    sin_theta: f64,
    cos_theta: f64,
    hittable: H,
    aabb: Option<AABB>,
}

impl<H: Hittable> Rotate<H> {
    pub fn new(axis: Axis, hittable: H, angle: f64) -> Rotate<H> {
        let (r_axis, a_axis, b_axis) = get_axis_index(&axis);
        let radiants = angle.to_radians();
        let sin_theta = radiants.sin();
        let cos_theta = radiants.cos();

        let aabb = hittable.bounding_box(0.0, 1.0).map(|mut aabb| {
            let mut min = Vec3::new(f64::MIN, f64::MIN, f64::MIN);
            let mut max = Vec3::new(f64::MAX, f64::MAX, f64::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r =
                            k as f64 * aabb.max.get(r_axis) + (1 - k) as f64 * aabb.min.get(r_axis);
                        let a =
                            i as f64 * aabb.max.get(a_axis) + (1 - i) as f64 * aabb.min.get(a_axis);
                        let b =
                            j as f64 * aabb.max.get(b_axis) + (1 - j) as f64 * aabb.min.get(b_axis);
                        let new_a = cos_theta * a + sin_theta * b;
                        let new_b = -sin_theta * a + cos_theta * b;

                        if new_a < min.get(a_axis) {
                            min.set(a_axis, new_a)
                        }
                        if new_b < min.get(b_axis) {
                            min.set(b_axis, new_b)
                        }
                        if r < min.get(r_axis) {
                            min.set(r_axis, r)
                        }

                        if new_a > max.get(a_axis) {
                            max.set(a_axis, new_a)
                        }
                        if new_b > max.get(b_axis) {
                            max.set(b_axis, new_b)
                        }
                        if r > max.get(r_axis) {
                            max.set(r_axis, r)
                        }
                    }
                }
            }
            aabb.min = min;
            aabb.max = max;
            aabb
        });
        Rotate {
            axis,
            sin_theta,
            cos_theta,
            hittable,
            aabb,
        }
    }
}

impl<H: Hittable> Hittable for Rotate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = get_axis_index(&self.axis);
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.set(
            a_axis,
            self.cos_theta * r.origin().get(a_axis) - self.sin_theta * r.origin().get(b_axis),
        );
        origin.set(
            b_axis,
            self.sin_theta * r.origin().get(a_axis) + self.cos_theta * r.origin().get(b_axis),
        );

        direction.set(
            a_axis,
            self.cos_theta * r.direction().get(a_axis)
                - self.sin_theta * r.direction().get(b_axis),
        );
        direction.set(
            b_axis,
            self.sin_theta * r.direction().get(a_axis)
                + self.cos_theta * r.direction().get(b_axis),
        );

        let rotated_ray = Ray::new(origin, direction, r.time());

        self.hittable
            .hit(&rotated_ray, t_min, t_max)
            .map(|mut hit| {
                let mut position = hit.position;
                let mut normal = hit.normal;

                position.set(
                    a_axis,
                    self.cos_theta * hit.position.get(a_axis)
                        + self.sin_theta * hit.position.get(b_axis),
                );
                position.set(
                    b_axis,
                    -self.sin_theta * hit.position.get(a_axis)
                        + self.cos_theta * hit.position.get(b_axis),
                );

                normal.set(
                    a_axis,
                    self.cos_theta * hit.normal.get(a_axis)
                        + self.sin_theta * hit.normal.get(b_axis),
                );
                normal.set(
                    b_axis,
                    -self.sin_theta * hit.normal.get(a_axis)
                        + self.cos_theta * hit.normal.get(b_axis),
                );

                hit.position = position;
                hit.set_face_normal(&rotated_ray, normal);
                hit
            })
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        self.aabb
    }
}
