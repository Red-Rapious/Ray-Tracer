use nalgebra::{Point3, Vector3};
use rand::{Rng, RngCore};
use real_interval::RealInterval;

use crate::world::{HitRecord, World};

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

    pub fn color(&self, depth: usize, world: &World, rng: &mut dyn RngCore) -> Vector3<f64> {
        let mut hit_record = HitRecord::default();

        // Max depth is exceeded, the ray will stop bouncing.
        if depth == 0 {
            return Vector3::from([0.0, 0.0, 0.0]);
        }

        if world.hit(
            self,
            RealInterval::min_max(0.001, f32::INFINITY), // 0.001 to limit "shadown acne"
            &mut hit_record,
        ) {
            //let direction = random_on_hemisphere(&hit_record.normal, rng);

            let direction = hit_record.normal + random_unit_vector(rng);
            let bouncing_ray = Ray::new(hit_record.hit_point, direction);
            0.5 * bouncing_ray.color(depth - 1, &world, rng)
        } else {
            // Display a blue gradient for background.
            let unit_direction = self.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);

            // Linear blue gradient
            Vector3::from([
                (1.0 - a) + a * 0.5,
                (1.0 - a) + a * 0.7,
                (1.0 - a) + a * 1.0,
            ])
        }
    }
}

/// Generates a random vector on the unit sphere.
fn random_unit_vector(rng: &mut dyn RngCore) -> Vector3<f64> {
    loop {
        let coords: [f64; 3] = [
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        ];
        let vector = Vector3::from(coords);
        if vector.norm_squared() >= 1.0 || vector.norm_squared() == 0.0 {
            continue;
        }

        return vector.normalize();
    }
}

/// Generates a random vector on the unit sphere on the same hemisphere as the given `normal`.
fn _random_on_hemisphere(normal: &Vector3<f64>, rng: &mut dyn RngCore) -> Vector3<f64> {
    let vector = random_unit_vector(rng);
    if vector.dot(normal) > 0.0 {
        return vector;
    } else {
        return -vector;
    }
}
