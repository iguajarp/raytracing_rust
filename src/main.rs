use std::{fs::File, io::Write};

mod color;
mod ppm;
mod progress_bar;
mod ray;
mod vec3;

use color::{write_color, Color};
use progress_bar::progress_bar;
use ray::Ray;
use vec3::Vec3;

fn ray_color(r: Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 256;
    let height = 256;
    let max_value = 255;

    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // camera & viewport
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // centers the ray to the center of the viewport
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0); // a bit more than 2.0
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2) - (&vertical / 2) - Vec3::new(0.0, 0.0, focal_length);

    let mut buffer = String::new();
    buffer.push_str(format!("P3\n{} {}\n{}\n", image_height, image_width, max_value).as_str());

    for j in (0..image_height).rev() {
        progress_bar((image_height - j) as usize, image_height as usize, 70);

        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let direction =
                &(&lower_left_corner + &(&horizontal * u)) + &(&(&vertical * v) - &origin);

            let r = Ray::new(&origin, &direction);
            let pxl_color: Color = ray_color(r);

            write_color(&mut buffer, pxl_color)
        }
    }
    let mut file = File::create("example.ppm").expect("error creating example.ppm");
    file.write_all(buffer.as_bytes())
        .expect("error writing to file");
    buffer.clear();

    println!("Done");
}
