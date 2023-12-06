use rand::seq::SliceRandom;

use crate::{
    geom::{ray::Ray, vec3::Vec3},
    hit::aabb::{surrounding_box, AABB},
    hit::hittable::{HitRecord, Hittable},
};

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.list.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut cloest_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, cloest_so_far) {
                cloest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut iter = self.list.iter();
        let first_bbox = iter.next()?.bounding_box(t0, t1)?;

        iter.try_fold(first_bbox, |acc, hitable| {
            hitable
                .bounding_box(t0, t1)
                .map(|bbox| surrounding_box(&acc, &bbox))
        })
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        self.list.iter().map(|h| h.pdf_value(o, v)).sum::<f64>() / self.list.len() as f64
    }

    fn random(&self, o: Vec3) -> Vec3 {
        self.list.choose(&mut rand::thread_rng()).unwrap().random(o)
    }
}
