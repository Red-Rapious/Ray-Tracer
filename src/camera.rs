use image::Rgba;
use nalgebra::{Point3, Vector3};

pub struct Camera {
    pub(crate) focal_length: f64,
    //pub(crate) viewport_height: f64,
    //pub(crate) viewport_width: f64,
    camera_center: Point3<f64>,
    pub(crate) samples_per_pixel: usize,
    pub(crate) max_depth: usize,
    pub(crate) vertical_fov: f64,
    gamma: Gamma,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        //viewport_height: f64,
        //actual_ratio: f64,
        camera_center: Point3<f64>,
        samples_per_pixel: usize,
        max_depth: usize,
        vertical_fov: f64,
        gamma: Gamma,
    ) -> Self {
        assert_ne!(samples_per_pixel, 0);

        Self {
            focal_length,
            //viewport_height,
            //viewport_width: viewport_height * actual_ratio,
            camera_center,
            samples_per_pixel,
            max_depth,
            vertical_fov,
            gamma,
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
