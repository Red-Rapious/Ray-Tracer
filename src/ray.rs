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

    pub fn color(&self, world: &World, rng: &mut dyn RngCore) -> Vector3<f64> {
        let mut hit_record = HitRecord::default();
        if world.hit(
            self,
            RealInterval::min_max(0.0, f32::INFINITY),
            &mut hit_record,
        ) {
            let direction = random_on_hemisphere(&hit_record.normal, rng);
            let bouncing_ray = Ray::new(hit_record.hit_point, direction);
            0.5 * bouncing_ray.color(&world, rng)
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

fn random_on_hemisphere(normal: &Vector3<f64>, rng: &mut dyn RngCore) -> Vector3<f64> {
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

        let vector = vector.normalize();

        if vector.dot(normal) > 0.0 {
            return vector;
        } else {
            return -vector;
        }
    }
}
