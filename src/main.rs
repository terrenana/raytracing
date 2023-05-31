use std::{fs::File, io::Write};

use crate::ray::*;
use glam::Vec3;

mod object;
mod ray;

// IMAGE
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = 400;
const HEIGHT: u32 = (WIDTH as f32 / ASPECT_RATIO) as u32;

// CAMERA
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

type Color = Vec3;
type Point = Vec3;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> Option<f32> {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius.powi(2);

    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

fn color_ray(r: Ray) -> Color {
    if let Some(t) = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &r) {
        let normal: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
    } else {
        let unit: Vec3 = r.direction.normalize();
        let t = 0.5 * (unit.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn write_color(c: Color, f: &mut File) -> std::io::Result<()> {
    f.write_all(
        &format!(
            "{} {} {}\n",
            (255.999 * c.x) as u8,
            (255.999 * c.y) as u8,
            (255.999 * c.z) as u8
        )
        .as_bytes(),
    )
}

fn main() -> std::io::Result<()> {
    let lower_left_corner =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut f = File::create("output.ppm")?;

    f.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())?;

    for j in (0..HEIGHT).rev() {
        println!("\rLines remaining: {}", j);
        for i in 0..WIDTH {
            let u = i as f32 / (WIDTH - 1) as f32;
            let v = j as f32 / (HEIGHT - 1) as f32;

            let ray = Ray::new(
                ORIGIN,
                lower_left_corner + u * HORIZONTAL + v * VERTICAL - ORIGIN,
            );

            let color: Color = color_ray(ray);
            write_color(color, &mut f)?;
        }
    }
    println!("\nDone.\n");

    Ok(())
}
