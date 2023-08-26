use crate::Ray;
use nalgebra::{Point3, Vector3};

/// An object hittable by a ray.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let origin_to_center = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = origin_to_center.dot(&ray.direction());
        let c = origin_to_center.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let discr_sqrt = discriminant.sqrt();
        let root = (-half_b - discr_sqrt) / a;
        if root <= t_min || t_max <= root {
            // First root is out of the allowed range
            let root = (-half_b + discr_sqrt) / a;
            if root <= t_min || t_max <= root {
                // Second root is out of the range
                return false; // No hit in the range
            }
        }

        hit_record.t = root;
        hit_record.hit_point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.hit_point - self.center) / self.radius;

        true
    }
}
