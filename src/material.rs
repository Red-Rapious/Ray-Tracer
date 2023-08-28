use crate::ray::Ray;
use crate::world::HitRecord;

use nalgebra::Vector3;
use rand::{Rng, RngCore};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Vector3<f64>),
    Hemisphere(Vector3<f64>),
    Metal(Vector3<f64>, f64),
}

impl Material {
    /// If the material is diffusive, returns `true` and modifies `scattered_ray` and `attenuation`.
    /// If the material isn't diffusive, returns `false`.
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered_ray: &mut Ray,
        rng: &mut dyn RngCore,
    ) -> bool {
        use Material::*;
        match *self {
            Lambertian(albedo) => {
                let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

                // Catch degenerate scatter direction
                if scatter_direction.norm_squared() < 1e-8 {
                    scatter_direction = hit_record.normal;
                }

                *scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);
                *attenuation = albedo;
                true
            }
            Hemisphere(albedo) => {
                let direction = random_on_hemisphere(&hit_record.normal, rng);
                *scattered_ray = Ray::new(hit_record.hit_point, direction);
                *attenuation = albedo;
                true
            }
            Metal(albedo, fuzz) => {
                let direction = reflect(ray_in.direction(), &hit_record.normal)
                    + fuzz * random_unit_vector(rng);
                *scattered_ray = Ray::new(hit_record.hit_point, direction);
                *attenuation = albedo;

                // If the scattered ray is below the surface, absorb it (return false)
                direction.dot(&hit_record.normal) > 0.0
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

/// Given a vector and a normal, returns the reflection of the vector on the surface represented by the normal.
fn reflect(vector: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    vector - 2.0 * vector.dot(normal) * normal
}
