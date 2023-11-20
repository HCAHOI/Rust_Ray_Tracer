use rand::Rng;

use crate::{
    color::Color,
    mat::{Dielectric, Lambertian, Metal},
    sphere::{MovingSphere, Sphere},
    vec3::{Point3, Vec3},
    world::World,
    world_add,
};

/// Select a exmaple scene
///
/// Choices:
/// - 1: Random scene
/// - 2: Random scene with moving spheres
/// - default: Random scene
pub fn scene_select(scene: u8) -> World {
    match scene {
        1 => random_scene(),
        2 => random_scene_with_move(),
        _ => random_scene(),
    }
}

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::default();

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world_add!(world, ground_sphere);

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random_range(0.0..1.0) * Color::random_range(0.0..1.0);
                let sphere_mat = Lambertian::new(albedo);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world_add!(world, sphere);
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random_range(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Metal::new(albedo, fuzz);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world_add!(world, sphere);
            } else {
                // Glass
                let sphere_mat = Dielectric::new(1.5);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world_add!(world, sphere);
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world_add!(world, sphere1);
    world_add!(world, sphere2);
    world_add!(world, sphere3);

    world
}

fn random_scene_with_move() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::default();

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world_add!(world, ground_sphere);

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random_range(0.0..1.0) * Color::random_range(0.0..1.0);
                let sphere_mat = Lambertian::new(albedo);
                let center1 =
                    center + Vec3::new(rng.gen_range(-0.1..0.1), rng.gen_range(0.0..0.5), 0.0);
                let sphere = MovingSphere::new(center, center1, 0.0, 1.0, 0.2, sphere_mat);

                world_add!(world, sphere);
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random_range(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Metal::new(albedo, fuzz);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world_add!(world, sphere);
            } else {
                // Glass
                let sphere_mat = Dielectric::new(1.5);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world_add!(world, sphere);
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world_add!(world, sphere1);
    world_add!(world, sphere2);
    world_add!(world, sphere3);

    world
}
