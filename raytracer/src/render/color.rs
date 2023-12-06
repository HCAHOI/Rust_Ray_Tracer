use crate::{geom::ray::Ray, geom::vec3::Vec3, hit::hittable::Hittable};

use super::mat::ScatterRecord;
use super::pdf::PDF;

pub type Color = Vec3;

impl Color {
    pub fn output(self, samples_per_pixel: u64) -> Self {
        let r = 256.0
            * (self.x / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999);
        let g = 256.0
            * (self.y / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999);
        let b = 256.0
            * (self.z / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999);

        Color::new(r, g, b)
    }
}

pub fn ray_color(
    ray: &Ray,
    background: Color,
    world: &Box<dyn Hittable>,
    lights: &Box<dyn Hittable>,
    depth: u64,
) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(ray, 0.00001, f64::INFINITY) {
        let emitted: Color = rec.material.emitted(&rec);
        if let Some(srec) = rec.material.scatter_monte_carlo(ray, &rec) {
            match srec {
                ScatterRecord::Specular {
                    specular_ray,
                    attenuation,
                } => {
                    return attenuation
                        * ray_color(&specular_ray, background, world, lights, depth - 1)
                }
                ScatterRecord::Scatter { pdf, attenuation } => {
                    let hittable_pdf = PDF::hittable_pdf(rec.position, lights);
                    let mixture_pdf = PDF::mixture_pdf(&hittable_pdf, &pdf);
                    let scattered = Ray::new(rec.position, mixture_pdf.generate(), ray.time());
                    let pdf_value = mixture_pdf.value(scattered.direction());
                    return emitted
                        + attenuation
                            * rec.material.scatter_pdf(ray, &rec, &scattered)
                            * ray_color(&scattered, background, world, lights, depth - 1)
                            / pdf_value;
                }
            }
        } else {
            emitted
        }
    } else {
        background
    }
}
