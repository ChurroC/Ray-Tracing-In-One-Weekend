mod vec3;
mod color;
mod ray;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(0.0, 0.0, 0.0) + t * Color::new(0.5, 0.7, 1.0)

}

fn main() {
    const IMAGE_HEIGHT: u16 = 256;
    const IMAGE_WIDTH: u16 = 256;

    print!("P3\n{IMAGE_HEIGHT} {IMAGE_WIDTH}\n255\n");

    for i in 0..IMAGE_WIDTH {
        eprint!("line {} - ", i + 1);
        for j in 0..IMAGE_HEIGHT {
            let r = i as f64 / (IMAGE_HEIGHT - 1) as f64; // So when you are at the right most pixel then r will be 1.0
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64; // So when you are at the bottom most pixel then g will be 1.0
            let b = 0.0;
            let pixel_color = Color::new(r, g, b);
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone!");
}
