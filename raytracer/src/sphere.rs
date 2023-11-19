use crate::material::Material;

use crate::utils::PI;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
