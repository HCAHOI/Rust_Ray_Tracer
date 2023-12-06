use rand::Rng;

use crate::hit::hittable::FlipNormal;
use crate::{
    cfg::ASPECT_RATIO,
    geom::quad::{Plane, Quad},
    geom::sphere::{MovingSphere, Sphere},
    geom::{
        cube::Cube,
        vec3::{Point3, Vec3},
    },
    hit::bvh::BVH,
    hit::hittable::Hittable,
    render::color::Color,
    render::mat::{Dielectric, DiffuseLight, Lambertian, Metal},
    render::texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture},
    transform::{
        rotate::{Axis, Rotate},
        translate::Translate,
    },
    world::camera::Camera,
    world::hittablelist::HittableList,
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
/// - 6: Final scene
/// - default: Random scene
pub fn scene_select(scene: u8) -> (Box<dyn Hittable>, Color, Camera) {
    match scene {
        1 => random_scene(),
        2 => two_perlin_sphere(),
        3 => earth_sphere(),
        4 => light_room(),
        5 => cornell_box(),
        6 => final_scene(),
        _ => random_scene(),
    }
}

fn random_scene() -> (Box<dyn Hittable>, Color, Camera) {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Box<dyn Hittable>> = vec![];

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

fn two_perlin_sphere() -> (Box<dyn Hittable>, Color, Camera) {
    let mut world = HittableList::default();

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

fn earth_sphere() -> (Box<dyn Hittable>, Color, Camera) {
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

fn light_room() -> (Box<dyn Hittable>, Color, Camera) {
    let mut world = HittableList::default();

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

fn cornell_box() -> (Box<dyn Hittable>, Color, Camera) {
    let mut world = HittableList::default();

    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(25.0, 25.0, 25.0)));

    world.push(Quad::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(Quad::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(Quad::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 0.0, white));
    world.push(Quad::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 555.0, white));
    world.push(Quad::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white));
    world.push(FlipNormal::new(Quad::new(
        Plane::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light,
    )));

    world.push(Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white,
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    world.push(Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));

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

fn final_scene() -> (Box<dyn Hittable>, Color, Camera) {
    let mut world = HittableList::default();

    let mut rng = rand::thread_rng();
    let ground = Lambertian::new(ConstantTexture::new(Color::new(0.48, 0.83, 0.53)));
    let mut box_list1: Vec<Box<dyn Hittable>> = Vec::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rng.gen::<f64>() + 0.01);
            let z1 = z0 + w;
            box_list1.push(Box::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground,
            )));
        }
    }
    world.push(BVH::new(box_list1, 0.0, 1.0));

    let light = DiffuseLight::new(ConstantTexture::new(Color::new(7.0, 7.0, 7.0)));
    world.push(Quad::new(
        Plane::XZ,
        147.0,
        412.0,
        123.0,
        423.0,
        554.0,
        light,
    ));

    let center = Point3::new(400.0, 400.0, 200.0);
    world.push(MovingSphere::new(
        center,
        center + Point3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Lambertian::new(ConstantTexture::new(Color::new(0.7, 0.3, 0.1))),
    ));
    world.push(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let image = image::open(
        "/home/hoi/Desktop/courses/2023-2024-1/Computer Graphics/labs/Rust_Ray_Tracer/img/SJTU-Badge.png",
    )
    .expect("image not found")
    .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    world.push(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        Lambertian::new(texture),
    ));
    world.push(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(NoiseTexture::new(0.1)),
    ));

    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let mut box_list2: Vec<Box<dyn Hittable>> = Vec::new();
    let ns = 1000;
    for _ in 0..ns {
        box_list2.push(Box::new(Sphere::new(
            Point3::new(
                165.0 * rng.gen::<f64>(),
                165.0 * rng.gen::<f64>(),
                165.0 * rng.gen::<f64>(),
            ),
            10.0,
            white,
        )));
    }
    world.push(Translate::new(
        Rotate::new(Axis::Y, BVH::new(box_list2, 0.0, 0.1), 15.0),
        Point3::new(-100.0, 270.0, 395.0),
    ));

    let bgcolor = Color::zero();

    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.01;
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
