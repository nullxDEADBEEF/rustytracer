mod vec3;

use vec3::{Color, Vec3};

fn main() {
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");

    for j in (0..IMAGE_HEIGHT - 1).rev() {
        eprintln!("\rScanlines remaning: {j}");
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = Vec3::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}
