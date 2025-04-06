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
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length); // Focal length just moves from eye to forward to the viewport

    print!("P3\n{IMAGE_HEIGHT} {IMAGE_WIDTH}\n255\n");

    for i in 0..IMAGE_WIDTH {
        eprint!("line {} - ", i + 1);
        for j in 0..IMAGE_HEIGHT {
            let u = i as f64 / (IMAGE_HEIGHT - 1) as f64; // So when you are at the right most pixel then r will be 1.0
            let v = j as f64 / (IMAGE_WIDTH - 1) as f64; // So when you are at the bottom most pixel then g will be 1.0
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            // u * horizontal gives a point to right of the plane of the viewport same with v * vertical
            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone!");
}
