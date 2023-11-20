use crate::{
    aabb::{surrounding_box, AABB},
    hit::{Hit, HitRecord},
    ray::Ray,
};

pub struct World {
    pub list: Vec<Box<dyn Hit>>,
}

impl Default for World {
    fn default() -> Self {
        World { list: vec![] }
    }
}

impl Hit for World {
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
