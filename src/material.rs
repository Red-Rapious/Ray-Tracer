use crate::ray::Ray;
use crate::world::HitRecord;

use nalgebra::Vector3;
use rand::{Rng, RngCore};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Vector3<f64>),
    Hemisphere(Vector3<f64>),
}

impl Material {
    pub fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
        rng: &mut dyn RngCore
    ) -> bool {
        use Material::*;
        match *self {
            Lambertian(albedo) => {
                let direction = hit_record.normal + random_unit_vector(rng);

                // TODO: catch degenerate scatter direction

                *scattered = Ray::new(hit_record.hit_point, direction);
                *attenuation = albedo;
                true
            },
            Hemisphere(albedo) => {
                let direction = random_on_hemisphere(&hit_record.normal, rng);
                *scattered = Ray::new(hit_record.hit_point, direction);
                *attenuation = albedo;
                true
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Vector3::new(0.5, 0.5, 0.5))
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
fn random_on_hemisphere(normal: &Vector3<f64>, rng: &mut dyn RngCore) -> Vector3<f64> {
    let vector = random_unit_vector(rng);
    if vector.dot(normal) > 0.0 {
        return vector;
    } else {
        return -vector;
    }
}