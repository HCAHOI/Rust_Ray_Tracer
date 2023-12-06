use rand::Rng;

use crate::geom::ray::Ray;
use crate::geom::vec3::Vec3;
use crate::hit::hittable::HitRecord;
use crate::render::color::Color;
use crate::render::onb::ONB;
use crate::render::pdf::PDF;
use crate::render::texture::Texture;
use crate::utils::PI;

pub enum ScatterRecord<'a> {
    Specular {
        specular_ray: Ray,
        attenuation: Color,
    },
    Scatter {
        pdf: PDF<'a>,
        attenuation: Color,
    },
}

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn scatter_monte_carlo(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter_pdf(&self, r_in: &Ray, rec: &HitRecord, ray_out: &Ray) -> f64 {
        0.0
    }
}

#[derive(Copy, Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.position, scatter_direction, r_in.time());

        Some((
            self.albedo.texture_map(rec.u, rec.v, &rec.position),
            scattered,
        ))
    }

    fn scatter_monte_carlo(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let rec = ScatterRecord::Scatter {
            pdf: PDF::cosine_pdf(rec.normal),
            attenuation: self.albedo.texture_map(rec.u, rec.v, &rec.position),
        };
        Some(rec)
    }

    fn scatter_pdf(&self, r_in: &Ray, rec: &HitRecord, ray_out: &Ray) -> f64 {
        rec.normal.dot(ray_out.direction().unit()).max(0.0) / PI
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal).unit();
        let scattered = Ray::new(
            rec.position,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }

    fn scatter_monte_carlo(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(r_in.direction(), rec.normal).unit();
        let scattered = Ray::new(
            rec.position,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );

        if scattered.direction().dot(rec.normal) > 0.0 {
            let rec = ScatterRecord::Specular {
                specular_ray: scattered,
                attenuation: self.albedo,
            };
            Some(rec)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ir: f64) -> f64 {
        let r0 = ((1.0 - ir) / (1.0 + ir)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();

        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.position, direction, r_in.time());
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }

    fn scatter_monte_carlo(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit();

        let cos_theta = ((-1.0) * unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.position, direction, r_in.time());

        let rec = ScatterRecord::Specular {
            specular_ray: scattered,
            attenuation,
        };

        Some(rec)
    }
}

#[derive(Copy, Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> DiffuseLight<T> {
        DiffuseLight { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, rec: &HitRecord) -> Color {
        if rec.front_face {
            self.emit.texture_map(rec.u, rec.v, &rec.position)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}
