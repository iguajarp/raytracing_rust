use std::{fs::File, io::Write};
mod vec3;
mod progress_bar;

use progress_bar::progress_bar;
use vec3::Vec3;

fn write_ppm(w: i32, h: i32, max_value: i32) {
    let mut buffer = String::new();
    buffer.push_str(format!("P3\n{} {}\n{}\n", w, h, max_value).as_str());

    for j in (0..h).rev() {
        progress_bar((h - j) as usize, h as usize, 70);

        for i in 0..w {
            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            buffer.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    let mut file = File::create("example.ppm").expect("error creating example.ppm");
    file.write_all(buffer.as_bytes())
        .expect("error writing to file");
}

fn main() {
    let width = 256;
    let height = 256;
    let max_value = 255;

    write_ppm(width, height, max_value);
}
