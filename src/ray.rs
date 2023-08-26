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

    pub fn at(&self, t: f64) -> Point3<f64> {
        assert!(t > 0.0);
        self.origin + t * self.direction
    }

    fn hit_sphere(&self, center: Point3<f64>, radius: f64) -> Option<f64> {
        let oc = self.origin - center;
        let a = self.direction.norm_squared();
        let half_b = oc.dot(&self.direction);
        let c = oc.norm_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }

    pub fn color(&self) -> Rgba<u8> {
        match self.hit_sphere(Point3::from([0.0, 0.0, -1.0]), 0.5) {
            Some(t) if t > 0.0 => {
                let normal_vector =
                    (self.at(t).coords - Vector3::from([0.0, 0.0, -1.0])).normalize();
                generate_color(
                    0.5 * (normal_vector.x + 1.0),
                    0.5 * (normal_vector.y + 1.0),
                    0.5 * (normal_vector.z + 1.0),
                )
            }
            _ => {
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
}

fn generate_color(red: f64, green: f64, blue: f64) -> Rgba<u8> {
    assert!(0.0 <= red && red <= 1.0);
    assert!(0.0 <= green && green <= 1.0);
    assert!(0.0 <= blue && blue <= 1.0);

    Rgba([
        (red * 255.999) as u8,
        (green * 255.999) as u8,
        (blue * 255.999) as u8,
        255,
    ])
}
