use rand::{thread_rng, Rng};
use real_interval::RealInterval;
use std::cmp::Ordering;

use crate::aabb::AABB;
use crate::geometry::Hittable;
use crate::ray::Ray;
use crate::world::HitRecord;

/// A Bounding Volume Hierarchy Node.
/// Each object is a node of the tree. The `left` and `right` hittables can be either other nodes or leafs.
pub struct BVHNode {
    /// The left node or leaf.
    left: Box<dyn Hittable + Sync>,
    /// The right node or leaf. The node may not have a right son, hence the use of `Option`.
    right: Option<Box<dyn Hittable + Sync>>,
    /// The bouding box encapsulating every son of the node.
    bbox: AABB,
}

impl BVHNode {
    /// Create a new Node from a vector of hittables. This function is recursively called.
    pub fn new(
        objects: &mut Vec<Option<Box<dyn Hittable + Sync>>>,
        start: usize,
        end: usize,
    ) -> Self {
        // Choose a random axis to compare the boxes.
        let axis = thread_rng().gen_range(0..3);

        let object_span = end - start;

        let left: Box<dyn Hittable + Sync>;
        let right: Option<Box<dyn Hittable + Sync>>;

        if object_span == 1 {
            left = std::mem::replace(&mut objects[start], None).unwrap();
            // If there is only one element, there is no right leaf.
            right = None;
        } else if object_span == 2 {
            // If there is exactly two elements, put each node as a leaf.
            match BVHNode::box_compare(&objects[start], &objects[start + 1], axis) {
                Ordering::Less | Ordering::Equal => {
                    left = std::mem::replace(&mut objects[start], None).unwrap();
                    right = std::mem::replace(&mut objects[start + 1], None); // gives initial `objects[start + 1]`
                }
                Ordering::Greater => {
                    left = std::mem::replace(&mut objects[start + 1], None).unwrap();
                    right = std::mem::replace(&mut objects[start], None); // gives initial `objects[start + 1]`
                }
            }
        } else {
            objects[start..end].sort_by(|a, b| BVHNode::box_compare(&a, &b, axis));

            let mid = start + object_span / 2;
            left = Box::new(BVHNode::new(objects, start, mid));
            right = Some(Box::new(BVHNode::new(objects, mid, end)));
        }

        let bbox = match &right {
            Some(right_node) => AABB::from_boxes(&left.bounding_box(), &right_node.bounding_box()),
            None => left.bounding_box().clone(),
        };

        Self { left, right, bbox }
    }

    /// Compare the coordinate of the boxes on the specified axis.
    fn box_compare(
        a: &Option<Box<dyn Hittable + Sync>>,
        b: &Option<Box<dyn Hittable + Sync>>,
        axis: usize,
    ) -> Ordering {
        match (a, b) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (Some(a), Some(b)) => a
                .bounding_box()
                .axis(axis)
                .min
                .partial_cmp(&b.bounding_box().axis(axis).min)
                .unwrap(),
        }
    }
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
        let hit_right = match &self.right {
            Some(right) => right.hit(
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
            ),
            None => false,
        };

        hit_left || hit_right
    }
}
