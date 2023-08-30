use nalgebra::Vector3;
use rand::{Rng, RngCore};

/// Generates a random vector on the unit sphere.
pub fn random_unit_vector(rng: &mut dyn RngCore) -> Vector3<f64> {
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
pub fn random_on_hemisphere(normal: &Vector3<f64>, rng: &mut dyn RngCore) -> Vector3<f64> {
    let vector = random_unit_vector(rng);
    if vector.dot(normal) > 0.0 {
        return vector;
    } else {
        return -vector;
    }
}

/// Generates a random vector inside the disk of radius 1 and with `z = 0`
pub fn random_in_unit_disk(rng: &mut dyn RngCore) -> Vector3<f64> {
    loop {
        let v = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
        if v.norm_squared() < 1.0 {
            return v;
        }
    }
}

/// Given a vector and a normal, returns the reflection of the vector on the surface represented by the normal.
pub fn reflect(vector: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    vector - 2.0 * vector.dot(normal) * normal
}

/// Given a **unit** vector, the normal of a diopter, and the ratio of the indices of both optical mediums,
/// computes the direction of the refracted ray after crossing the diopter.
pub fn refract(
    unit_vector: &Vector3<f64>,
    normal: &Vector3<f64>,
    indices_ratio: f64,
) -> Vector3<f64> {
    let cos_theta = -unit_vector.dot(normal);

    let ray_out_perp = indices_ratio * (unit_vector + cos_theta * normal);
    let ray_out_parallel = -(1.0 - ray_out_perp.norm_squared()).abs().sqrt() * normal;

    ray_out_perp + ray_out_parallel
}

/// Use Schlick's approximation to compute the reflectance.
pub fn reflectance(cosine: f64, indices_ratio: f64) -> f64 {
    let r0 = (1.0 - indices_ratio) / (1.0 + indices_ratio);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

/// A 2-dimensional basis, containing 2 base vectors, `u` and `v`.
pub struct Basis2<T> {
    pub u: Vector3<T>,
    pub v: Vector3<T>,
}

impl<T> Basis2<T> {
    pub fn new(u: Vector3<T>, v: Vector3<T>) -> Self {
        Self { u, v }
    }
}

/// A 3-dimensional basis, containing 3 base vectors, `u`, `v`, and `w`.
pub struct Basis3<T> {
    pub u: Vector3<T>,
    pub v: Vector3<T>,
    pub w: Vector3<T>,
}

impl<T> Basis3<T> {
    pub fn new(u: Vector3<T>, v: Vector3<T>, w: Vector3<T>) -> Self {
        Self { u, v, w }
    }
}
