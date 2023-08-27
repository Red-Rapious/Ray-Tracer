use crate::material::Material;
use crate::ray::Ray;
use crate::world::HitRecord;

use nalgebra::Point3;
use real_interval::RealInterval;

/// An object hittable by a ray.
pub trait Hittable {
    /// Check if the given ray hits the hittable. If so, it adds informations about the hit to `hit_record`.
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool;
}

/// A basic Sphere geometry.
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool {
        let origin_to_center = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = origin_to_center.dot(&ray.direction());
        let c = origin_to_center.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // No real root: there's no hit
        if discriminant < 0.0 {
            return false;
        }

        // There might be a hit; checks if the hit(s) is/are in the allowed interval for t.
        let discr_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discr_sqrt) / a;

        // First root is out of the allowed interval
        if root <= t_interval.min as f64 || t_interval.max as f64 <= root {
            root = (-half_b + discr_sqrt) / a; // second root

            // Second root is out of the range
            if root <= t_interval.min as f64 || t_interval.max as f64 <= root {
                return false; // No hit in the interval
            }
        }

        // Modify the hit record accordingly
        hit_record.t = root;
        hit_record.hit_point = ray.at(hit_record.t);
        hit_record.material = self.material;

        let outward_normal = (hit_record.hit_point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        true // there's a hit
    }
}
