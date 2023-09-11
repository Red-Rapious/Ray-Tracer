use nalgebra::{Point3, Vector3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3<f64>) -> Vector3<f64>;
}

pub struct SolidColor {
    color_value: Vector3<f64>,
}

impl SolidColor {
    pub fn new(color_value: Vector3<f64>) -> Self {
        Self { color_value }
    }

    pub fn rgb(red: f64, blue: f64, green: f64) -> Self {
        Self { color_value: Vector3::new(red, blue, green) }
    }
} 

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3<f64>) -> Vector3<f64> {
        self.color_value
    }
}