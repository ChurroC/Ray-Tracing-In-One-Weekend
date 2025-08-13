mod vec3;
mod color;
mod ray;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};


fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> Option<f64> {
    let orig_cen = center - r.origin();
    let a = r.direction().dot(r.direction());
    let half_b = orig_cen.dot(r.direction());
    let c = orig_cen.dot(orig_cen) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((half_b - f64::sqrt(discriminant)) / a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let sphere_origin = Point3::new(0.0, 0.0, -1.0);
    // At whats t value does the ray hit the sphere
    if let Some(t) = hit_sphere(sphere_origin, 0.5, r) {
        // normal is also radius vector
        let normal = (r.at(t) - sphere_origin).unit_vector();
        return 0.5 * Color::new_vec(normal + Vec3::new(1.0, 1.0, 1.0));
    }

    let unit_dir = r.direction().unit_vector();
    // Dot product gives y component of the direction vector from -1 pointing down to 1 pointing up
    // +1 shifts from -1 to 1 bounds to 0 to 2 bounds and divided by 2 gives 0 to 1 bounds
    // Top is Color::new(1.0, 1.0, 1.0) and bottom is Color::new(0.5, 0.7, 1.0)
    // Though the ray would never point just up so t never gets to 1 or -1
    let t = 0.5 * (unit_dir.dot(Vec3::new(0.0, 1.0, 0.0)) + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Just in general pos z is coming at you and neg is away
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    eprintln!("Rendering image of size {} x {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    // We calc ASPECT_RATIO again through IMAGE_WIDTH / IMAGE_HEIGHT to get actual ratio and not ideal
    let viewport_width = (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64) * viewport_height;
    // Distance from the eye to the viewport kinda like how far away your standing almost
    // Like focal distance on camera from lik 24mm to 100mm
    let focal_length = 1.0;

    // Where eye is located and starting point of the ray
    let origin = Point3::new(0.0, 0.0, 0.0);
    // We use the u and v vectors to represent the viewport plane
    // u goes to the right and v goes down so (0, 0, 0) is the top left corner of the viewport
    // I was thinking of using the image_height values but then realized that this is so it is in relation to the z value of 1
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;
    
    // Focal length just moves from eye to forward to the viewport (-z since that's toward viewport) or moves the z to the plane
    // Focal length is also basically the 2d image the camera sees
    let upper_left_corner = origin - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0 ;
    // This is the center of the pixel at 0,0
    let pixel_00_center = upper_left_corner + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for v in 0..IMAGE_HEIGHT {
        for u in 0..IMAGE_WIDTH {
            // u * horizontal gives a point to right of the plane of the viewport same with v * vertical
            let pixel_center =  pixel_00_center + pixel_delta_u * u as f64 + pixel_delta_v * v as f64;
            let r = Ray::new(
                origin,
                pixel_center - origin,
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
            //eprint!("Finished row {} and col {}\r", v + 1, u + 1);
        }
    }
    eprintln!("\nDone!");
}
