#![allow(dead_code)]
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(buff: &mut String, pixel_color: Color) {
    buff.push_str(format!(
        "{} {} {}\n",
        255.99 * pixel_color.x(),
        255.99 * pixel_color.y(),
        255.99 * pixel_color.z(),
    ).as_str())
}
