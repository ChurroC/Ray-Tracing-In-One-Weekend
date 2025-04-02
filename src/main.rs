mod vec3;
mod color;

use vec3::Point3;
use color::Color;


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
