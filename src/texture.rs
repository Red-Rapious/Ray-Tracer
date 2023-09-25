use nalgebra::{Point3, Vector3};
use image::{DynamicImage, GenericImageView};

#[derive(Clone, Copy)]
pub enum Texture {
    SolidColor(Vector3<f64>),
    Checker(f64, &'static Texture, &'static Texture),
    Image(&'static DynamicImage)
}

impl Texture {
    /// Computes the color of the texture at the given point `p`.
    /// The color also depends on the mapping on the texture, given by the parameters `u` and `v`.
    pub fn value(&self, u: f64, v: f64, p: Point3<f64>) -> Vector3<f64> {
        match *self {
            Self::SolidColor(color) => color,
            Self::Checker(inv_scale, color_even, color_odd) => {
                let x_int = (inv_scale * p.x) as i32;
                let y_int = (inv_scale * p.y) as i32;
                let z_int = (inv_scale * p.z) as i32;

                if (x_int + y_int + z_int) % 2 == 0 {
                    color_even.value(u, v, p)
                } else {
                    color_odd.value(u, v, p)
                }
            },
            Self::Image(image) => {
                if image.height() == 0 {
                    // Cyan for debugging purposes
                    return Vector3::new(0.0, 1.0, 1.0);
                }

                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let x = (u * image.width() as f64) as u32;
                let y = (v * image.height() as f64) as u32;
                let pixel = image.get_pixel(x, y);

                let color_scale = 1.0 / 255.0;
                Vector3::new(pixel[0] as f64 * color_scale, pixel[1] as f64 * color_scale, pixel[2] as f64 * color_scale)
            }
        }
    }
}
