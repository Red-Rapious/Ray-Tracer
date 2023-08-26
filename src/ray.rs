use image::Rgba;
use nalgebra::{Point3, Vector3};

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

    fn generate_color(red: f64, green: f64, blue: f64) -> Rgba<u8> {
        Rgba([(red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8, 255])
    }

    pub fn color(&self) -> Rgba<u8> {
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        Self::generate_color(
            (1.0 - a) + a * 0.5,
            (1.0 - a) + a * 0.7,
            (1.0 - a) + a * 1.0,
        )
    }
}
