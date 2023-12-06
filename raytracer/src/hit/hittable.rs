use crate::geom::ray::Ray;
use crate::geom::vec3::{Point3, Vec3};
use crate::hit::aabb::AABB;
use crate::render::mat::Material;

pub struct HitRecord<'a> {
    pub position: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64, // texture coordinates
    pub v: f64, // texture coordinates
    pub front_face: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        }
    }
}

#[derive(Clone)]
pub struct FlipNormal<H: Hittable> {
    hittable: H,
}

impl<H: Hittable> FlipNormal<H> {
    pub fn new(flipped: H) -> FlipNormal<H> {
        FlipNormal { hittable: flipped }
    }
}

impl<H: Hittable> Hittable for FlipNormal<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hittable.hit(&r, t_min, t_max).map(|mut rec| {
            rec.front_face = !rec.front_face;
            rec
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }

    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        self.hittable.pdf_value(o, v)
    }

    fn random(&self, o: Vec3) -> Vec3 {
        self.hittable.random(o)
    }
}
