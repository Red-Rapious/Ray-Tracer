use nalgebra::{Point3, Vector3};

#[derive(Clone, Copy)]
pub enum Texture {
    SolidColor(Vector3<f64>),
    CheckerTexture(f64, &'static Texture, &'static Texture),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Point3<f64>) -> Vector3<f64> {
        match *self {
            Self::SolidColor(color) => color,
            Self::CheckerTexture(inv_scale, color_even, color_odd) => {
                let x_int = (inv_scale * p.x) as i32;
                let y_int = (inv_scale * p.y) as i32;
                let z_int = (inv_scale * p.z) as i32;

                if (x_int + y_int + z_int) % 2 == 0 {
                    color_even.value(u, v, p)
                } else {
                    color_odd.value(u, v, p)
                }
            }
        }
    }
}
