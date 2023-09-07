use crate::utility::{random_in_unit_disk, Basis2, Basis3};

use image::Rgba;
use nalgebra::{Point3, Vector3};
use rand::RngCore;

pub struct Camera {
    /// The simulated focus distance. Objects at this distance from the Camera won't be affected by defocus blur.
    pub(crate) focus_distance: f64,
    /// The number of rays send per pixel.
    pub(crate) samples_per_pixel: usize,
    /// The maximum number of times that a ray can bounce.
    pub(crate) max_depth: usize,
    /// The vertical field of view, in degrees.
    pub(crate) vertical_fov: f64,
    /// A 3D orthonormal basis describing the viewport.
    pub(crate) frame_basis: Basis3<f64>,
    /// A 2D basis describing the defocus disk.
    pub(crate) disk_basis: Basis2<f64>,
    /// The center of the camera, the point from where the rays are emitted.
    pub center: Point3<f64>,
    /// The type of gamma correction applied to the image.
    gamma: Gamma,
}

impl Camera {
    pub fn new(
        samples_per_pixel: usize,
        max_depth: usize,
        vertical_fov: f64,
        look_from: Point3<f64>,
        look_at: Point3<f64>,
        up_direction: Vector3<f64>,
        gamma: Gamma,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        assert_ne!(samples_per_pixel, 0);
        assert!((0.0..360.0).contains(&vertical_fov));
        assert!((0.0..360.0).contains(&defocus_angle));
        assert_ne!(look_from, look_at);
        assert!(focus_distance > 0.0);

        let w = (look_from - look_at).normalize();
        let u = up_direction.cross(&w).normalize();
        let v = w.cross(&u);

        let frame_basis = Basis3::new(u, v, w);

        // Calculate the camera defocus disk basis
        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let disk_basis = Basis2::new(defocus_radius * u, defocus_radius * v);

        Self {
            focus_distance,
            samples_per_pixel,
            max_depth,
            vertical_fov,
            gamma,
            frame_basis,
            center: look_from,
            disk_basis,
        }
    }

    /// Converts a vector of 3 floats to a color, `image::Rgba<u8>`.
    pub fn color_to_pixel(&self, color: Vector3<f64>) -> Rgba<u8> {
        match self.gamma {
            Gamma::Gamma2 => Rgba([
                (color.x.sqrt() * 255.0) as u8,
                (color.y.sqrt() * 255.0) as u8,
                (color.z.sqrt() * 255.0) as u8,
                255,
            ]),
        }
    }

    pub fn defocus_disk_sample(&self, rng: &mut dyn RngCore) -> Point3<f64> {
        if self.disk_basis.u == Vector3::zeros() {
            self.center
        } else {
            let p = random_in_unit_disk(rng);
            self.center + self.disk_basis.u * p.x + self.disk_basis.v * p.y
        }
    }
}

pub enum Gamma {
    Gamma2,
}
