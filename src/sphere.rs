use crate::{hittable::{Hittable, HitRecord}, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let oc: Vec3 = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = &oc.length_squared() - self.radius * self.radius;

        // the discriminant determines if the rect intersect with the sphere. If positive it does, if 0 tangent, negative not.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (&rec.p - &self.center) / self.radius;

        return true;
    }
}
