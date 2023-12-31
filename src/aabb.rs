use nalgebra::Point3;
use real_interval::RealInterval;

use crate::ray::Ray;

/// Axis-Aligned Bounding Box
#[derive(Debug, Default, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    pub(crate) x: RealInterval,
    pub(crate) y: RealInterval,
    pub(crate) z: RealInterval,
}

impl AABB {
    pub fn new(x: RealInterval, y: RealInterval, z: RealInterval) -> Self {
        Self { x, y, z }
    }

    /// Creates an AABB that contains both points.
    pub fn from_points(a: Point3<f64>, b: Point3<f64>) -> Self {
        Self {
            x: RealInterval {
                min: a.x.min(b.x) as f32,
                max: a.x.max(b.x) as f32,
            },
            y: RealInterval {
                min: a.y.min(b.y) as f32,
                max: a.y.max(b.y) as f32,
            },
            z: RealInterval {
                min: a.z.min(b.z) as f32,
                max: a.z.max(b.z) as f32,
            },
        }
    }

    /// Creates an AABB that contains both boxes.
    pub fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: RealInterval {
                min: box0.x.min.min(box1.x.min),
                max: box0.x.max.max(box1.x.max),
            },
            y: RealInterval {
                min: box0.y.min.min(box1.y.min),
                max: box0.y.max.max(box1.y.max),
            },
            z: RealInterval {
                min: box0.z.min.min(box1.z.min),
                max: box0.z.max.max(box1.z.max),
            },
        }
    }

    /// Converts an `axis` integer into the corresponding axis.
    pub fn axis(&self, axis: usize) -> &RealInterval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }

    pub fn hit(&self, ray: &Ray, t_interval: RealInterval) -> bool {
        let mut t_interval = t_interval;
        for axis in 0..3 {
            let d_invert = 1.0 / ray.direction()[axis];
            let origin = ray.origin()[axis];

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
