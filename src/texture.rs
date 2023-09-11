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
                let x_int = (inv_scale * p.x) as usize;
                let y_int = (inv_scale * p.y) as usize;
                let z_int = (inv_scale * p.z) as usize;

                if (x_int + y_int + z_int % 2) == 0 {
                    color_even.value(u, v, p)
                } else {
                    color_odd.value(u, v, p)
                }
            }
        }
    }
}

/*pub trait Texture {
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
        Self {
            color_value: Vector3::new(red, blue, green),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3<f64>) -> Vector3<f64> {
        self.color_value
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn from_colors(scale: f64, color_even: Vector3<f64>, color_odd: Vector3<f64>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(SolidColor::new(color_even)),
            odd: Box::new(SolidColor::new(color_odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3<f64>) -> Vector3<f64> {
        let x_int = (self.inv_scale * p.x) as usize;
        let y_int = (self.inv_scale * p.y) as usize;
        let z_int = (self.inv_scale * p.z) as usize;

        if (x_int + y_int + z_int % 2) == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}*/