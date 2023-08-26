use image::Rgba;
use nalgebra::{Point3, Vector3};
use real_interval::RealInterval;

use crate::world::{HitRecord, World};

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

    pub fn at(&self, t: f64) -> Point3<f64> {
        assert!(t > 0.0);
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &World) -> Rgba<u8> {
        let mut hit_record = HitRecord::default();
        if world.hit(
            self,
            RealInterval::min_max(0.0, f32::INFINITY),
            &mut hit_record,
        ) {
            generate_color(
                0.5 * (hit_record.normal.x + 1.0),
                0.5 * (hit_record.normal.y + 1.0),
                0.5 * (hit_record.normal.z + 1.0),
            )
        } else {
            let unit_direction = self.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);

            generate_color(
                (1.0 - a) + a * 0.5,
                (1.0 - a) + a * 0.7,
                (1.0 - a) + a * 1.0,
            )
        }
    }
}

fn generate_color(red: f64, green: f64, blue: f64) -> Rgba<u8> {
    assert!(0.0 <= red && red <= 1.0, "red = {red}");
    assert!(0.0 <= green && green <= 1.0, "green = {green}");
    assert!(0.0 <= blue && blue <= 1.0, "blue = {blue}");

    Rgba([
        (red * 255.999) as u8,
        (green * 255.999) as u8,
        (blue * 255.999) as u8,
        255,
    ])
}
