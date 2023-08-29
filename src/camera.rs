use image::Rgba;
use nalgebra::{Point3, Vector3};

pub struct Camera {
    pub(crate) focal_length: f64,
    pub(crate) samples_per_pixel: usize,
    pub(crate) max_depth: usize,
    pub(crate) vertical_fov: f64,
    //pub(crate) look_from: Point3<f64>,
    //pub(crate) look_at: Point3<f64>,
    //pub(crate) up_direction: Vector3<f64>,
    pub(crate) basis: Basis<f64>,
    camera_center: Point3<f64>,
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
    ) -> Self {
        assert_ne!(samples_per_pixel, 0);

        let focal_length = (look_from - look_at).norm();
        let w = (look_from - look_at).normalize();
        let u = up_direction.cross(&w).normalize();
        let v = w.cross(&u);

        let basis = Basis::new(u, v, w);

        Self {
            focal_length,
            samples_per_pixel,
            max_depth,
            vertical_fov,
            //look_from,
            //look_at,
            //up_direction,
            gamma,
            basis,
            camera_center: look_from,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.camera_center
    }

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
}

pub enum Gamma {
    Gamma2,
}

#[derive(Debug)]
pub struct Basis<T> {
    pub u: Vector3<T>,
    pub v: Vector3<T>,
    pub w: Vector3<T>,
}

impl<T> Basis<T> {
    pub fn new(u: Vector3<T>, v: Vector3<T>, w: Vector3<T>) -> Self {
        Self { u, v, w }
    }
}
