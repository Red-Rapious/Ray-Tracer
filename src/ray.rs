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

    fn hit_sphere(&self, center: Point3<f64>, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = self.direction.norm_squared();
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.norm_squared() - radius*radius;
        let discriminant = b*b - 4.0*a*c;

        discriminant >= 0.0
    }

    pub fn color(&self) -> Rgba<u8> {
        if self.hit_sphere(Point3::from([0.0, 0.0, -1.0]), 0.5) {
            generate_color(1.0, 0.0, 0.0)
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
    Rgba([
        (red * 255.0) as u8,
        (green * 255.0) as u8,
        (blue * 255.0) as u8,
        255,
    ])
}
