use rand::Rng;

use crate::{
    geom::vec3::{Point3, Vec3},
    hit::hittable::Hittable,
    utils::PI,
};

use super::onb::ONB;

pub enum PDF<'a> {
    Cosine {
        uvw: ONB,
    },
    Hittable {
        origin: Point3,
        hittable: &'a Box<dyn Hittable>,
    },
    Mixture {
        p0: &'a PDF<'a>,
        p1: &'a PDF<'a>,
    },
}

impl<'a> PDF<'a> {
    pub fn cosine_pdf(w: Vec3) -> PDF<'a> {
        PDF::Cosine {
            uvw: ONB::build_from_w(&w),
        }
    }

    pub fn hittable_pdf(origin: Point3, hittable: &'a Box<dyn Hittable>) -> PDF<'a> {
        PDF::Hittable { origin, hittable }
    }

    pub fn mixture_pdf(p0: &'a PDF, p1: &'a PDF) -> PDF<'a> {
        PDF::Mixture { p0, p1 }
    }

    pub fn value(&self, direction: Vec3) -> f64 {
        match self {
            PDF::Cosine { uvw } => {
                let cosine = direction.unit().dot(uvw.w());
                if cosine > 0.0 {
                    // importance sampling
                    cosine / PI
                } else {
                    0.0
                }
            }
            PDF::Hittable { origin, hittable } => hittable.pdf_value(*origin, direction),
            PDF::Mixture { p0, p1 } => 0.5 * p0.value(direction) + 0.5 * p1.value(direction),
        }
    }

    pub fn generate(&self) -> Vec3 {
        match self {
            PDF::Cosine { uvw } => uvw.local(&Vec3::random_cos_direction()),
            PDF::Hittable { origin, hittable } => hittable.random(*origin),
            PDF::Mixture { p0, p1 } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    p0.generate()
                } else {
                    p1.generate()
                }
            }
        }
    }
}
