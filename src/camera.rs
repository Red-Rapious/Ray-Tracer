use nalgebra::Point3;

pub struct Camera {
    pub(crate) focal_length: f64,
    pub(crate) viewport_height: f64,
    pub(crate) viewport_width: f64,
    camera_center: Point3<f64>,
}

impl Camera {
    pub fn new(
        focal_length: f64,
        viewport_height: f64,
        actual_ratio: f64,
        camera_center: Point3<f64>,
    ) -> Self {
        Self {
            focal_length,
            viewport_height,
            viewport_width: viewport_height * actual_ratio,
            camera_center,
        }
    }

    pub fn center(&self) -> &Point3<f64> {
        &self.camera_center
    }
}
