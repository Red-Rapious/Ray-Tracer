use crate::aabb::AABB;
use crate::geometry::Hittable;
use crate::ray::Ray;
use crate::world::HitRecord;
use real_interval::RealInterval;

pub struct BVHNode {
    left: Box<dyn Hittable + Sync>,
    right: Box<dyn Hittable + Sync>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn hit(&self, ray: &Ray, t_interval: RealInterval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_interval) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_interval, hit_record);
        let hit_right = self.right.hit(
            ray,
            RealInterval {
                min: t_interval.min,
                max: if hit_left {
                    hit_record.t as f32
                } else {
                    t_interval.max
                },
            },
            hit_record,
        );

        hit_left || hit_right
    }
}
