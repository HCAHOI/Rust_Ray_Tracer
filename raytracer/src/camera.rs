use rand::Rng;

use crate::utils::PI;

use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use std::f64;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        // vertical field-of-view in degrees
        let theta = PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = viewport_height * aspect_ratio;

        let cw = (lookfrom - lookat).unit();
        let cu = Vec3::cross(vup, cw).unit();
        let cv = Vec3::cross(cw, cu);

        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;
        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
            cu,
            cv,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x + self.cv * rd.y;

        //track time between the time of strat and end
        let time = self.time0 + rand::thread_rng().gen::<f64>() * (self.time1 - self.time0);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical
                - (self.origin + offset),
            time,
        )
    }
}
