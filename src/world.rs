use crate::ray::Ray;
use nalgebra::{Point3, Vector3};
use real_interval::RealInterval;

/// An object hittable by a ray.
pub trait Hittable {
    /// Check if the given ray hits the hittable. If so, it adds informations about the hit to `hit_record`.
    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool;
}

#[derive(Default, Clone)]
pub struct HitRecord {
    pub hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(hit_point: Point3<f64>, normal: Vector3<f64>, t: f64, front_face: bool) -> Self {
        Self {
            hit_point,
            normal,
            t,
            front_face,
        }
    }

    /// Sets the hit record normal vector. `outward_normal` is assumed to be normalised.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn empty() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    /// Check if the given ray hits any hittable from the `objects` list.
    /// If so, it adds the information of the closest hit to `hit_record`
    pub fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool {
        let mut temporary_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t_interval.max as f64;

        for object in self.objects.iter() {
            if object.hit(
                ray,
                RealInterval::min_max(t_interval.min, closest as f32),
                &mut temporary_record,
            ) {
                hit_anything = true;
                closest = temporary_record.t;
                *hit_record = temporary_record.clone();
            }
        }

        hit_anything
    }
}
