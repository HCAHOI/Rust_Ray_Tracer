mod aabb;
mod bvh;
mod camera;
mod cfg;
mod color;
mod hit;
mod mat;
mod perlin;
mod quad;
mod ray;
mod scene;
mod sphere;
mod texture;
mod utils;
mod vec3;
mod world;

use crate::{color::ray_color, scene::scene_select};
use cfg::*;
use color::Color;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    // World
    let (world, bgcolor, camera) = scene_select(SCENE_SELECTOR);

    // Image
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_sample| {
                    let mut rng = rand::thread_rng();
                    let random_u = rng.gen::<f64>();
                    let random_v = rng.gen::<f64>();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = camera.get_ray(u, v);
                    ray_color(&r, &world, bgcolor, MAX_DEPTH)
                })
                .sum();
            let pixel = img.get_pixel_mut(i as u32, (IMAGE_HEIGHT - j - 1) as u32);
            let pixel_color = pixel_color.output(SAMPLES_PER_PIXEL);
            *pixel = image::Rgb([
                pixel_color.x as u8,
                pixel_color.y as u8,
                pixel_color.z as u8,
            ]);
        }
        bar.inc(1);
    }

    bar.finish();
    img.save("output/test.png").unwrap();
}
