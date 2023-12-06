use crate::geom::vec3::Point3;
use crate::{geom::ray::Ray, geom::vec3::Vec3, hit::hittable::Hittable};

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

pub fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, color: Color, depth: u64) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 1e-5, f64::INFINITY) {
        let emitted: Color = rec.material.emitted(&rec);
        let on_light = Point3::new(255.0, 554.0, 277.0); // Set according to the light in the Cornell Box scene
        let mut to_light = on_light - rec.position;
        let distance_squared = to_light.length().powi(2);

        if to_light.length() > 0.0 {
            to_light = to_light.unit();
        }
        if to_light.dot(rec.normal) < 0.0 {
            return emitted;
        }

        let light_area = (343.0 - 213.0) * (332.0 - 227.0); // Set according to the light in the Cornell Box scene
        let light_cos = to_light.y.abs();
        if light_cos < 1e-6 {
            return emitted;
        }

        if let Some((attenuation, mut ray_out, mut pdf)) =
            rec.material.scatter_monte_carlo(ray, &rec)
        {
            pdf = distance_squared / (light_cos * light_area);
            ray_out = Ray::new(rec.position, to_light, ray.time());
            emitted
                + attenuation
                    * rec.material.scatter_pdf(ray, &rec, &ray_out)
                    * ray_color(&ray_out, world, color, depth - 1)
                    / pdf
        } else {
            emitted
        }
    } else {
        color
    }
}
