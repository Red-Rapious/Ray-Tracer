use nalgebra::Vector3;
use rand::{Rng, RngCore};

use crate::ray::Ray;
use crate::texture::Texture;
use crate::utility::*;
use crate::world::HitRecord;

// TODO: transform Material into a trait
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Vector3<f64>),
    TexturedLambertian(Texture),
    Hemisphere(Vector3<f64>),
    Metal(Vector3<f64>, f64),
    Dielectric(f64),
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

                *scattered_ray = Ray::new(hit_record.hit_point, scatter_direction, ray_in.time());
                *attenuation = albedo;
                true
            }
            TexturedLambertian(texture) => {
                let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

                // Catch degenerate scatter direction
                if scatter_direction.norm_squared() < 1e-8 {
                    scatter_direction = hit_record.normal;
                }

                *scattered_ray = Ray::new(hit_record.hit_point, scatter_direction, ray_in.time());
                *attenuation = texture.value(hit_record.u, hit_record.v, hit_record.hit_point);
                true
            }
            Hemisphere(albedo) => {
                let direction = random_on_hemisphere(&hit_record.normal, rng);
                *scattered_ray = Ray::new(hit_record.hit_point, direction, ray_in.time());
                *attenuation = albedo;
                true
            }
            Metal(albedo, fuzz) => {
                let direction = reflect(ray_in.direction(), &hit_record.normal)
                    + fuzz * random_unit_vector(rng);
                *scattered_ray = Ray::new(hit_record.hit_point, direction, ray_in.time());
                *attenuation = albedo;

                // If the scattered ray is below the surface, absorb it (return false)
                direction.dot(&hit_record.normal) > 0.0
            }
            Dielectric(index) => {
                *attenuation = Vector3::new(1.0, 1.0, 1.0);
                let refraction_ratio = match hit_record.front_face {
                    true => 1.0 / index, // ray goes from air to the dielectric
                    false => index,      // ray goes from the dielectric to the air
                };

                let unit_direction = ray_in.direction().normalize();

                let cos_theta = -unit_direction.dot(&hit_record.normal);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                // If sin(theta) is too big, there's total reflexion
                let direction = match refraction_ratio * sin_theta <= 1.0
                    || reflectance(cos_theta, refraction_ratio) > rng.gen()
                {
                    true => refract(&unit_direction, &hit_record.normal, refraction_ratio),
                    false => reflect(&unit_direction, &hit_record.normal),
                };

                *scattered_ray = Ray::new(hit_record.hit_point, direction, ray_in.time());
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
