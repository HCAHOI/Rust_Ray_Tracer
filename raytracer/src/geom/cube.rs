use crate::{
    render::mat::Material,
    hit::{
        aabb::AABB,
        hittable::{HitRecord, Hittable},
    },
    world::hittablelist::HittableList,
};

use super::{
    quad::{Plane, Quad},
    ray::Ray,
    vec3::Point3,
};

pub struct Cube {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new<M: Material + Clone + 'static>(min: Point3, max: Point3, material: M) -> Cube {
        let mut sides = HittableList::default();

        sides.push(Quad::new(
            Plane::XY,
            min.x,
            max.x,
            min.y,
            max.y,
            max.z,
            material.clone(),
        ));
        sides.push(Quad::new(
            Plane::XY,
            min.x,
            max.x,
            min.y,
            max.y,
            min.z,
            material.clone(),
        ));

        sides.push(Quad::new(
            Plane::XZ,
            min.x,
            max.x,
            min.z,
            max.z,
            max.y,
            material.clone(),
        ));
        sides.push(Quad::new(
            Plane::XZ,
            min.x,
            max.x,
            min.z,
            max.z,
            min.y,
            material.clone(),
        ));

        sides.push(Quad::new(
            Plane::YZ,
            min.y,
            max.y,
            min.z,
            max.z,
            max.x,
            material.clone(),
        ));
        sides.push(Quad::new(
            Plane::YZ,
            min.y,
            max.y,
            min.z,
            max.z,
            min.x,
            material,
        ));

        Cube { min, max, sides }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.min,
            max: self.max,
        })
    }
}
