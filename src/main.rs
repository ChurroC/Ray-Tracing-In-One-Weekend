mod vec3;

use vec3::Vec3;

fn main() {
    const IMAGE_HEIGHT: u16 = 256;
    const IMAGE_WIDTH: u16 = 256;

    print!("P3\n{IMAGE_HEIGHT} {IMAGE_WIDTH}\n255\n");

    for i in 0..IMAGE_WIDTH {
        eprint!("line {} done - ", i + 1);
        for j in 0..IMAGE_HEIGHT {
            let r = i as f64 / (IMAGE_HEIGHT - 1) as f64; // So when you are at the right most pixel then r will be 1.0
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64; // So when you are at the bottom most pixel then g will be 1.0
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            print!("{ir} {ig} {ib}\n");
        }
    }
    eprintln!("\nDone!");

    let v = Vec3::new(1.0, 2.0, 3.0);

    let mut wow = v * 2.0;
    wow *= 2.0;
    wow -= Vec3::new(1.0, 2.0, 3.0);
    let wowww = wow.dot(Vec3::new(1.0, 1.0, 1.0));
    eprintln!("{wowww}");
}
