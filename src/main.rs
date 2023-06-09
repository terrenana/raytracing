use std::rc::Rc;

use crate::camera::Camera;
use crate::material::*;
use crate::object::ObjectList;
use crate::ray::*;
use crate::util::*;
use glam::Vec3;
use image::ImageBuffer;
use image::Rgb;
use object::*;
use rand::random;

mod camera;
mod material;
mod object;
mod ray;
mod util;

// IMAGE
const ASPECT_RATIO: f32 = 3.0 / 2.0;
const WIDTH: u32 = 200;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;

type Color = Vec3;

fn main() -> image::ImageResult<()> {
    let camera_position = Vec3::new(13.0, 2.0, 3.0);
    let camera_pointing_at = Vec3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let camera = Camera::new(
        camera_position,
        camera_pointing_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        focus_distance,
    );

    let world = random_scene();

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH, HEIGHT);

    for (i, (x, y, pixel)) in image.enumerate_pixels_mut().enumerate() {
        print!("\x1B[2J");
        println!(
            "RENDER PROGRESS:\n{:.2}% ({} out of {} pixels)",
            (i as f64 / (WIDTH * HEIGHT) as f64) * 100.0,
            i,
            WIDTH * HEIGHT
        );
        let mut color: Color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f32 + random::<f32>()) / (WIDTH - 1) as f32;
            let v = ((HEIGHT - y) as f32 + random::<f32>()) / (HEIGHT - 1) as f32;
            let ray = camera.get_ray(u, v);
            color += color_ray(&ray, &world, MAX_DEPTH);
            write_color(color, pixel, SAMPLES_PER_PIXEL);
        }
    }

    image.save("output.png")?;

    println!("\nDone.\n");

    Ok(())
}
fn color_ray<O: Object>(r: &Ray, world: &O, depth: u32) -> Color {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(r, 0.001, f32::INFINITY) {
        match hit.material.scatter(r, &hit) {
            Some((ray, attenuation)) => attenuation * color_ray(&ray, world, depth - 1),
            None => Vec3::new(0.0, 0.0, 0.0),
        }
    } else {
        let unit: Vec3 = r.direction.normalize();
        let t = 0.5 * (unit.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(c: Color, pixel: &mut Rgb<u8>, samples: u32) {
    let scale = 1.0 / samples as f32;

    let r = (clamp(f32::sqrt(c.x * scale), 0.0, 0.999) * 256.0) as u8;
    let g = (clamp(f32::sqrt(c.y * scale), 0.0, 0.999) * 256.0) as u8;
    let b = (clamp(f32::sqrt(c.z * scale), 0.0, 0.999) * 256.0) as u8;

    let rgb = image::Rgb([r, g, b]);
    *pixel = rgb;
}

fn random_scene() -> ObjectList {
    let mut world = ObjectList::new(Vec::new());

    let ground_material = Rc::new(Lambertian::new(Vec3::new(0.25, 0.25, 0.25)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat_random = random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_random < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>());
                    let material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else if mat_random < 0.95 {
                    // metal
                    let albedo = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>());
                    let fuzz = random::<f32>();
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}
