use crate::geom::ray::Ray;
use crate::geom::vec3::{Point3, Vec3};
use crate::hit::aabb::{self, AABB};
use crate::hit::hittable::{HitRecord, Hittable};
use crate::render::mat::Material;
use crate::render::onb::ONB;
use crate::utils::PI;

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    ((p.z.atan2(p.x) + PI) / (2.0 * PI), p.y.acos() / PI)
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            position: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (rec.position - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        let (u, v) = get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let min = self.center - Vec3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vec3::new(self.radius, self.radius, self.radius);

        Some(AABB { min, max })
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if let Some(_) = self.hit(&Ray::new(o, v, 0.0), 0.001, f64::MAX) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - o).length().powi(2)).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length().powi(2);
        let uvw = ONB::build_from_w(&direction);
        uvw.local(&Vec3::random_to_sphere(self.radius, distance_squared))
    }
}

pub struct MovingSphere<M: Material> {
    center_st: Point3,
    center_ed: Point3,
    radius: f64,
    material: M,
    time_st: f64,
    time_ed: f64,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center_st: Point3,
        center_ed: Point3,
        time_st: f64,
        time_ed: f64,
        radius: f64,
        material: M,
    ) -> Self {
        MovingSphere {
            center_st,
            center_ed,
            radius,
            material,
            time_st,
            time_ed,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center_st
            + (time - self.time_st) / (self.time_ed - self.time_st)
                * (self.center_ed - self.center_st)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            position: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (rec.position - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);

        let (u, v) = get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);

        let box0 = AABB::new(self.center_st - radius_vec, self.center_st + radius_vec);
        let box1 = AABB::new(self.center_ed - radius_vec, self.center_ed + radius_vec);

        Some(aabb::surrounding_box(&box0, &box1))
    }
}
