use nalgebra::{Point3, Vector3};
use real_interval::RealInterval;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::world::HitRecord;

/// An object hittable by a ray.
pub trait Hittable {
    /// Check if the given ray hits the hittable. If so, it adds informations about the hit to `hit_record`.
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> &AABB;
    fn get_uv_coordinates(&self, point: Point3<f64>, u: &mut f64, v: &mut f64);
}

/// A basic Sphere geometry.
pub struct Sphere {
    center1: Point3<f64>,
    radius: f64,
    material: Material,
    is_moving: bool,
    center_vec: Vector3<f64>,
    bbox: AABB,
}

impl Sphere {
    pub fn stationary(center: Point3<f64>, radius: f64, material: Material) -> Self {
        let radius_vector = Vector3::new(radius, radius, radius);
        Self {
            center1: center,
            radius,
            material,
            is_moving: false,
            center_vec: Vector3::zeros(),
            bbox: AABB::from_points(center - radius_vector, center + radius_vector),
        }
    }

    pub fn moving(
        center1: Point3<f64>,
        center2: Point3<f64>,
        radius: f64,
        material: Material,
    ) -> Self {
        let radius_vector = Vector3::new(radius, radius, radius);
        let bbox1 = AABB::from_points(center1 - radius_vector, center1 + radius_vector);
        let bbox2 = AABB::from_points(center2 - radius_vector, center2 + radius_vector);

        Self {
            center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: AABB::from_boxes(&bbox1, &bbox2),
        }
    }

    fn center(&self, time: f64) -> Point3<f64> {
        if self.is_moving {
            self.center1 + time * self.center_vec
        } else {
            self.center1
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool {
        let origin_to_center = ray.origin() - self.center(ray.time());
        let a = ray.direction().norm_squared();
        let half_b = origin_to_center.dot(ray.direction());
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

        let outward_normal = (hit_record.hit_point - self.center(ray.time())) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        true // there's a hit
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn get_uv_coordinates(&self, point: Point3<f64>, u: &mut f64, v: &mut f64) {
        let theta = f64::acos(-point.y);
        let phi = f64::atan2(-point.z, point.x) + std::f64::consts::PI;

        *u = phi / (2.0 * std::f64::consts::PI);
        *v = theta / std::f64::consts::PI;
    }
}
