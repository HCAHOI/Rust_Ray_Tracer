use rand::Rng;

use crate::{
    cfg::ASPECT_RATIO,
    color::color::Color,
    color::mat::{Dielectric, DiffuseLight, Lambertian, Metal},
    color::texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture},
    geom::quad::{Plane, Quad},
    geom::sphere::{MovingSphere, Sphere},
    geom::vec3::{Point3, Vec3},
    hit::bvh::BVH,
    hit::hit::Hit,
    world::camera::Camera,
    world::world::World,
    world_add,
};

/// Select a exmaple scene
///
/// Choices:
/// - 1: Random scene
/// - 2: Two perlin sphere
/// - 3: Earth sphere
/// - 4: Light room
/// - 5: Cornell box
/// - default: Random scene
pub fn scene_select(scene: u8) -> (Box<dyn Hit>, Color, Camera) {
    match scene {
        1 => random_scene(),
        2 => two_perlin_sphere(),
        3 => earth_sphere(),
        4 => light_room(),
        5 => cornell_box(),
        _ => random_scene(),
    }
}

fn random_scene() -> (Box<dyn Hit>, Color, Camera) {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Box<dyn Hit>> = vec![];

    let ground_mat = Lambertian::new(CheckerTexture::new(
        ConstantTexture::new(Color::new(1.0, 1.0, 1.0)),
        ConstantTexture::new(Color::new(0.3, 1.0, 0.3)),
    ));
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
                let sphere_mat = Lambertian::new(ConstantTexture::new(albedo));
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
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
    let mat2 = Lambertian::new(ConstantTexture::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world_add!(world, sphere1);
    world_add!(world, sphere2);
    world_add!(world, sphere3);

    let bgcolor = Color::new(0.7, 0.8, 1.0);

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (Box::new(BVH::new(world, 0.0, 1.0)), bgcolor, camera)
}

fn two_perlin_sphere() -> (Box<dyn Hit>, Color, Camera) {
    let mut world = World::default();

    let top_mat = Lambertian::new(NoiseTexture::new(2.0));
    let bottom_mat = Lambertian::new(NoiseTexture::new(2.0));

    let top_sphere = Sphere::new(Point3::new(1000.0, 2.0, 1000.0), 2.0, top_mat);
    let bottom_sphere = Sphere::new(Point3::new(1000.0, -1000.0, 1000.0), 1000.0, bottom_mat);

    world.list.push(Box::new(top_sphere));
    world.list.push(Box::new(bottom_sphere));

    let bgcolor = Color::new(0.7, 0.8, 1.0);

    let lookfrom = Point3::new(1013.0, 2.0, 1003.0);
    let lookat = Point3::new(1000.0, 0.0, 1000.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (Box::new(world), bgcolor, camera)
}

fn earth_sphere() -> (Box<dyn Hit>, Color, Camera) {
    let image = image::open(
        "/home/hoi/Desktop/courses/2023-2024-1/Computer Graphics/labs/Rust_Ray_Tracer/img/e.jpg",
    )
    .expect("image not found")
    .to_rgb8();
    let (width, height) = image.dimensions();
    let img_data = image.into_raw();
    let texture = ImageTexture::new(img_data, width, height);
    let world = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));

    let bgcolor = Color::new(0.7, 0.8, 1.0);

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (Box::new(world), bgcolor, camera)
}

fn light_room() -> (Box<dyn Hit>, Color, Camera) {
    let mut world = World::default();

    let bottom_mat = Lambertian::new(ConstantTexture::new(Color::new(0.7, 0.7, 0.7)));
    let top_mat = Lambertian::new(NoiseTexture::new(2.0));
    let emitted = DiffuseLight::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)));

    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, bottom_mat);
    let sphere = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, top_mat);
    let plane = Quad::new(Plane::XY, 3.0, 5.0, 1.0, 3.0, -2.0, emitted);

    world.push(ground);
    world.push(sphere);
    world.push(plane);

    let bgcolor = Color::new(0.0, 0.0, 0.0);

    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (Box::new(world), bgcolor, camera)
}

fn cornell_box() -> (Box<dyn Hit>, Color, Camera) {
    let mut world = World::default();

    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(15.0, 15.0, 15.0)));

    world.push(Quad::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(Quad::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(Quad::new(
        Plane::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light,
    ));
    world.push(Quad::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ));
    world.push(Quad::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    world.push(Quad::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white));

    let bgcolor = Color::new(0.0, 0.0, 0.0);

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.05;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (Box::new(world), bgcolor, camera)
}
