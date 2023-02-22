use crate::{vec3::Vec3, ray::Ray};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}