use crate::ray::Ray;
use crate::world::{HitRecord, Hittable};
use nalgebra::Point3;
use real_interval::RealInterval;

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool {
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
        if root <= t_interval.min as f64 || t_interval.max as f64 <= root {
            // First root is out of the allowed range
            let root = (-half_b + discr_sqrt) / a;
            if root <= t_interval.min as f64 || t_interval.max as f64 <= root {
                // Second root is out of the range
                return false; // No hit in the range
            }
        }

        hit_record.t = root;
        hit_record.hit_point = ray.at(hit_record.t);
        let outward_normal = (hit_record.hit_point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        true
    }
}
