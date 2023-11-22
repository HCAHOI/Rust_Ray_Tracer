use crate::{
    geom::ray::Ray,
    hit::aabb::{surrounding_box, AABB},
    hit::hit::{HitRecord, Hittable},
};

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.list.push(Box::new(object));
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList { list: vec![] }
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
}
