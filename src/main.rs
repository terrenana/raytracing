use std::{fs::File, io::Write};

use crate::vector::*;

mod ray;
mod vector;

const HEIGHT: u32 = 256;
const WIDTH: u32 = 256;

fn main() -> std::io::Result<()> {
    let mut f = File::create("output.ppm")?;

    f.write_all(format!("P3\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())?;

    for j in (0..HEIGHT).rev() {
        println!("\rLines remaining: {}", j);
        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH - 1) as f64;
            let g = j as f64 / (HEIGHT - 1) as f64;
            let b = 0.25;

            let color: Color = Color::new(r, g, b);

            write_color(color, &mut f)?;
        }
    }
    println!("\nDone.\n");

    Ok(())
}
