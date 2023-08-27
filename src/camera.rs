use nalgebra::Point3;

pub struct Camera {
    pub(crate) focal_length: f64,
    pub(crate) viewport_height: f64,
    pub(crate) viewport_width: f64,
    camera_center: Point3<f64>,
    pub(crate) samples_per_pixel: usize,
    pub(crate) max_depth: usize,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        actual_ratio: f64,
        camera_center: Point3<f64>,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        assert_ne!(samples_per_pixel, 0);

        Self {
            focal_length,
            viewport_height,
            viewport_width: viewport_height * actual_ratio,
            camera_center,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.camera_center
    }
}
