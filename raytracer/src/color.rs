use crate::{hit::Hit, ray::Ray, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    // change range
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

pub fn ray_color(ray: &Ray, world: &Box<dyn Hit>, depth: u64) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.00001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
