#![allow(dead_code)]
/*
 * P(t)=A+tb . Here P is a 3D position along a line in 3D.
 * A  is the ray origin and b is the ray direction.
 * The ray parameter t is a real number (double in the code).
 * Plug in a different t and P(t) moves the point along the ray.
 * Add in negative t values and you can go anywhere on the 3D line.
 * For positive t, you get only the parts in front of A, and this is what is often called a half-line or ray.
 *
 * This is usually used to represent the ray from the camera in ray tracing.
 */

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray<'a> {
    orig: &'a Vec3,
    dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(orig: &'a Vec3, dir: &'a Vec3) -> Ray<'a> {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + &(self.dir * t)
    }
}
