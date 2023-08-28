use nalgebra::{Point3, Vector3};
use rand::RngCore;
use real_interval::RealInterval;

use crate::world::{HitRecord, World};

#[derive(Default)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3<f64> {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        assert!(t > 0.0, "t = {t}");
        self.origin + t * self.direction
    }

    /// Computes the color of the surface hit by the ray.
    pub fn color(&self, depth: usize, world: &World, rng: &mut dyn RngCore) -> Vector3<f64> {
        let mut hit_record = HitRecord::default();

        // Max depth is exceeded, the ray will stop bouncing.
        if depth == 0 {
            return Vector3::zeros();
        }

        // If the ray hit an object
        if world.hit(
            self,
            RealInterval::min_max(0.001, f32::INFINITY), // 0.001 to limit "shadown acne"
            &mut hit_record,
        ) {
            let mut bouncing_ray = Ray::default();
            let mut attenuation = Vector3::default();
            let material = hit_record.material;

            // If the
            if material.scatter(self, &hit_record, &mut attenuation, &mut bouncing_ray, rng) {
                attenuation.component_mul(&bouncing_ray.color(depth - 1, &world, rng))
            } else {
                Vector3::zeros()
            }
        } else {
            // Display a blue gradient for background.
            let unit_direction = self.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);

            // Linear blue gradient
            (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}
