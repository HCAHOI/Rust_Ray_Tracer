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

    if let Some(rec) = world.hit(ray, 0.00001, f64::INFINITY) {
        let emitted: Color = rec.material.emitted(rec.u, rec.v, &rec.position);
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            attenuation * ray_color(&scattered, world, color, depth - 1) + emitted
        } else {
            emitted
        }
    } else {
        color
    }
}
