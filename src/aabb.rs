use nalgebra::Point3;
use real_interval::RealInterval;

use crate::ray::Ray;

/// Axis-Aligned Bounding Box
#[derive(Default)]
pub struct AABB {
    pub(crate) x: RealInterval,
    pub(crate) y: RealInterval,
    pub(crate) z: RealInterval,
}

impl AABB {
    pub fn new(x: RealInterval, y: RealInterval, z: RealInterval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point3<f64>, b: &Point3<f64>) -> Self {
        Self {
            x: RealInterval {
                min: a.x.min(b.x) as f32,
                max: a.x.max(b.x) as f32,
            },
            y: RealInterval {
                min: a.x.min(b.y) as f32,
                max: a.x.max(b.y) as f32,
            },
            z: RealInterval {
                min: a.x.min(b.z) as f32,
                max: a.x.max(b.z) as f32,
            },
        }
    }

    fn axis(&self, axis: usize) -> &RealInterval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }

    pub fn hit(&self, r: &Ray, t_interval: &mut RealInterval) -> bool {
        for axis in 0..3 {
            let d_invert = 1.0 / r.direction()[axis];
            let origin = r.origin()[axis];

            let mut t0 = ((self.axis(axis).min as f64 - origin) * d_invert) as f32;
            let mut t1 = ((self.axis(axis).max as f64 - origin) * d_invert) as f32;

            if d_invert < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > t_interval.min {
                t_interval.min = t0;
            }

            if t1 < t_interval.max {
                t_interval.max = t1;
            }

            if t_interval.max <= t_interval.min {
                return false;
            }
        }

        true
    }
}
