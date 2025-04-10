mod vec3;
mod color;
mod ray;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_dir = r.direction().unit_vector();
    // Plus one to take it from -1 to 1 to 0 to 2 and then divide by 2 to take it from 0 to 1
    // But also y never really gets to the max 1
    // Also since it's a unit vector as we get further in the x the component of y gets smaller and different colors on edge compared to middle
    // Just switched to dot product to project how much of vec is in the y direction
    let t = 0.5 * (unit_dir.dot(Vec3::new(0.0, 1.0, 0.0)) + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let orig_cen = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * orig_cen.dot(r.direction());
    let c = orig_cen.dot(orig_cen) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn main() {
    // Just in general pos z is coming at you and neg is away

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    eprintln!("Rendering image of size {} x {}\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    // We calc ASPECT_RATIO again through IMAGE_WIDTH / IMAGE_HEIGHT to get actual ratio and not ideal
    let viewport_width = (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64) * viewport_height;
    // This is where all the light from viewport converges or how far away from the viewport you are
    let focal_length = 1.0;

    // Where eye is located
    let origin = Point3::new(0.0, 0.0, 0.0);
    // We use the u and v vectors to represent the viewport plane
    // u goes to the right and v goes down
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;
    
    // Focal length just moves from eye to forward to the viewport or moves the z to the plane
    let upper_left_corner = origin - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    // This is the center of the pixel at 0,0
    let pixel_00_center = upper_left_corner + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for v in 0..IMAGE_HEIGHT {
        // eprint!("line {} - ", v + 1);
        for u in 0..IMAGE_WIDTH {
            // u * horizontal gives a point to right of the plane of the viewport same with v * vertical
            let pixel_center =  pixel_00_center + pixel_delta_u * u as f64 + pixel_delta_v * v as f64;
            let r = Ray::new(
                origin,
                pixel_center - origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }
    eprintln!("\nDone!");
}
